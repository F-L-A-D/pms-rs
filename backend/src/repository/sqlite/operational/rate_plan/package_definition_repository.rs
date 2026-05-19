use sqlx::{Row, Sqlite, Transaction};

use crate::{
    domain::{
        entity::package_definition::{PackageDefinition, RatePlanDefinition, RatePlanPackage},
        semantic::reservation_booking::ReservationRevenueCategory,
    },
    error::app_error::{infra, AppResult},
};

pub struct SqlitePackageDefinitionRepository;

impl SqlitePackageDefinitionRepository {
    pub async fn save_rate_plan(
        tx: &mut Transaction<'_, Sqlite>,
        rate_plan: &RatePlanDefinition,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO rate_plan_definitions (
                plan_code,
                display_name,
                is_active
            )
            VALUES (?1, ?2, ?3)
            "#,
        )
        .bind(&rate_plan.plan_code)
        .bind(&rate_plan.display_name)
        .bind(rate_plan.is_active)
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_rate_plan_by_code(
        tx: &mut Transaction<'_, Sqlite>,
        plan_code: &str,
    ) -> AppResult<Option<RatePlanDefinition>> {
        let row = sqlx::query(
            r#"
            SELECT
                plan_code,
                display_name,
                is_active
            FROM rate_plan_definitions
            WHERE plan_code = ?1
            "#,
        )
        .bind(plan_code)
        .fetch_optional(&mut **tx)
        .await
        .map_err(infra)?;

        match row {
            Some(row) => Ok(Some(Self::row_to_rate_plan(&row)?)),
            None => Ok(None),
        }
    }

    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        package: &PackageDefinition,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO package_definitions (
                package_code,
                display_name,
                revenue_category,
                department_code,
                account_code,
                is_active
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
        )
        .bind(&package.package_code)
        .bind(&package.display_name)
        .bind(package.revenue_category.to_snake())
        .bind(&package.department_code)
        .bind(&package.account_code)
        .bind(package.is_active)
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_by_package_code(
        tx: &mut Transaction<'_, Sqlite>,
        package_code: &str,
    ) -> AppResult<Option<PackageDefinition>> {
        let row = sqlx::query(
            r#"
            SELECT
                package_code,
                display_name,
                revenue_category,
                department_code,
                account_code,
                is_active
            FROM package_definitions
            WHERE package_code = ?1
            "#,
        )
        .bind(package_code)
        .fetch_optional(&mut **tx)
        .await
        .map_err(infra)?;

        match row {
            Some(row) => Ok(Some(Self::row_to_package(&row)?)),
            None => Ok(None),
        }
    }

    pub async fn assign_to_plan(
        tx: &mut Transaction<'_, Sqlite>,
        assignment: &RatePlanPackage,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO rate_plan_packages (
                plan_code,
                package_code,
                sort_order
            )
            VALUES (?1, ?2, ?3)
            "#,
        )
        .bind(&assignment.plan_code)
        .bind(&assignment.package_code)
        .bind(assignment.sort_order)
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn list_by_plan_code(
        tx: &mut Transaction<'_, Sqlite>,
        plan_code: &str,
    ) -> AppResult<Vec<RatePlanPackage>> {
        let rows = sqlx::query(
            r#"
            SELECT
                plan_code,
                package_code,
                sort_order
            FROM rate_plan_packages
            WHERE plan_code = ?1
            ORDER BY sort_order, package_code
            "#,
        )
        .bind(plan_code)
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.iter().map(Self::row_to_plan_package).collect()
    }

    fn row_to_package(row: &sqlx::sqlite::SqliteRow) -> AppResult<PackageDefinition> {
        let revenue_category = ReservationRevenueCategory::from_snake(
            row.get::<String, _>("revenue_category").as_str(),
        )
        .ok_or_else(|| infra("invalid package revenue category"))?;

        Ok(PackageDefinition {
            package_code: row.get("package_code"),
            display_name: row.get("display_name"),
            revenue_category,
            department_code: row.get("department_code"),
            account_code: row.get("account_code"),
            is_active: row.get("is_active"),
        })
    }

    fn row_to_rate_plan(row: &sqlx::sqlite::SqliteRow) -> AppResult<RatePlanDefinition> {
        Ok(RatePlanDefinition {
            plan_code: row.get("plan_code"),
            display_name: row.get("display_name"),
            is_active: row.get("is_active"),
        })
    }

    fn row_to_plan_package(row: &sqlx::sqlite::SqliteRow) -> AppResult<RatePlanPackage> {
        Ok(RatePlanPackage {
            plan_code: row.get("plan_code"),
            package_code: row.get("package_code"),
            sort_order: row.get("sort_order"),
        })
    }
}
