use std::collections::HashMap;

use chrono::{Datelike, Duration, NaiveDate, Utc};

use rust_decimal::Decimal;

use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::aggregate::{
        access::fetch_daily_room_class_kpi_aggregate_rows::fetch_daily_room_class_kpi_aggregate_rows,
        model::monthly_room_class_kpi_aggregate::MonthlyRoomClassKpiAggregate,
    },
};

pub async fn materialize_monthly_room_class_kpi_aggregates(
    tx: &mut Transaction<'_, Sqlite>,
    year_month: &str,
) -> AppResult<Vec<MonthlyRoomClassKpiAggregate>> {
    let mut rows_by_class: HashMap<String, MonthlyRoomClassTotals> = HashMap::new();

    for service_date in month_dates(year_month)? {
        for row in fetch_daily_room_class_kpi_aggregate_rows(tx, service_date).await? {
            let totals = rows_by_class.entry(row.room_class).or_default();

            totals.total_room_nights += row.total_rooms;
            totals.out_of_order_room_nights += row.out_of_order_rooms;
            totals.reservable_room_nights += row.total_rooms - row.out_of_order_rooms;
            totals.sold_room_nights += row.sold_room_nights;
            totals.occupied_room_nights += row.occupied_rooms;
            totals.room_revenue += row.room_revenue;
            totals.food_and_beverage_revenue += row.food_and_beverage_revenue;
            totals.other_revenue += row.other_revenue;
            totals.tax_amount += row.tax_amount;
        }
    }

    let mut aggregates = rows_by_class
        .into_iter()
        .map(|(room_class, totals)| {
            let total_revenue = totals.room_revenue
                + totals.food_and_beverage_revenue
                + totals.other_revenue
                + totals.tax_amount;

            MonthlyRoomClassKpiAggregate {
                year_month: year_month.to_string(),
                room_class,
                total_room_nights: totals.total_room_nights,
                out_of_order_room_nights: totals.out_of_order_room_nights,
                reservable_room_nights: totals.reservable_room_nights,
                sold_room_nights: totals.sold_room_nights,
                occupied_room_nights: totals.occupied_room_nights,
                room_revenue: totals.room_revenue,
                food_and_beverage_revenue: totals.food_and_beverage_revenue,
                other_revenue: totals.other_revenue,
                tax_amount: totals.tax_amount,
                total_revenue,
                occupancy_rate: divide(totals.sold_room_nights, totals.reservable_room_nights),
                adr: divide_decimal(totals.room_revenue, totals.sold_room_nights),
                revpar: divide_decimal(totals.room_revenue, totals.reservable_room_nights),
                projection_version: 1,
                updated_at: Utc::now(),
            }
        })
        .collect::<Vec<_>>();

    aggregates.sort_by(|left, right| left.room_class.cmp(&right.room_class));

    Ok(aggregates)
}

#[derive(Default)]
struct MonthlyRoomClassTotals {
    total_room_nights: i64,
    out_of_order_room_nights: i64,
    reservable_room_nights: i64,
    sold_room_nights: i64,
    occupied_room_nights: i64,
    room_revenue: Decimal,
    food_and_beverage_revenue: Decimal,
    other_revenue: Decimal,
    tax_amount: Decimal,
}

fn month_dates(year_month: &str) -> AppResult<Vec<NaiveDate>> {
    let first_date = NaiveDate::parse_from_str(&format!("{year_month}-01"), "%Y-%m-%d")
        .map_err(crate::error::app_error::infra)?;
    let next_month = if first_date.month() == 12 {
        NaiveDate::from_ymd_opt(first_date.year() + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(first_date.year(), first_date.month() + 1, 1).unwrap()
    };

    let mut dates = Vec::new();
    let mut current = first_date;

    while current < next_month {
        dates.push(current);
        current += Duration::days(1);
    }

    Ok(dates)
}

fn divide(numerator: i64, denominator: i64) -> Decimal {
    if denominator == 0 {
        Decimal::ZERO
    } else {
        Decimal::from(numerator) / Decimal::from(denominator)
    }
}

fn divide_decimal(numerator: Decimal, denominator: i64) -> Decimal {
    if denominator == 0 {
        Decimal::ZERO
    } else {
        numerator / Decimal::from(denominator)
    }
}
