INSERT INTO daily_room_class_kpi_aggregates (
            service_date,
            room_class,
            total_rooms,
            out_of_order_rooms,
            reservable_rooms,
            sold_room_nights,
            occupied_rooms,
            room_revenue,
            food_and_beverage_revenue,
            other_revenue,
            tax_amount,
            total_revenue,
            occupancy_rate,
            adr,
            revpar,
            projection_version,
            updated_at
        )
        VALUES (
            ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
        )
        ON CONFLICT(service_date, room_class)
        DO UPDATE SET
            total_rooms                 = excluded.total_rooms,
            out_of_order_rooms          = excluded.out_of_order_rooms,
            reservable_rooms            = excluded.reservable_rooms,
            sold_room_nights            = excluded.sold_room_nights,
            occupied_rooms              = excluded.occupied_rooms,
            room_revenue                = excluded.room_revenue,
            food_and_beverage_revenue   = excluded.food_and_beverage_revenue,
            other_revenue               = excluded.other_revenue,
            tax_amount                  = excluded.tax_amount,
            total_revenue               = excluded.total_revenue,
            occupancy_rate              = excluded.occupancy_rate,
            adr                         = excluded.adr,
            revpar                      = excluded.revpar,
            projection_version          = excluded.projection_version,
            updated_at                  = excluded.updated_at
