use axum::{extract::State, Json};
use chrono::NaiveDate;

use crate::{
    auth::AuthUser,
    error::AppResult,
    models::{CategorySummary, MonthlySummary},
    AppState,
};

pub async fn get_monthly_summary(
    State(state): State<AppState>,
    user: AuthUser,
) -> AppResult<Json<Vec<MonthlySummary>>> {
    let summaries = sqlx::query_as::<_, MonthlySummary>(
        r#"
        SELECT
            TO_CHAR(expense_date, 'Month') as month,
            EXTRACT(YEAR FROM expense_date)::INTEGER as year,
            SUM(amount) as total_amount,
            COUNT(*)::BIGINT as expense_count
        FROM expenses
        WHERE user_id = $1
        GROUP BY month, year
        ORDER BY year DESC, MIN(expense_date) DESC
        LIMIT 12
        "#,
    )
    .bind(user.user_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(summaries))
}

pub async fn get_category_summary(
    State(state): State<AppState>,
    user: AuthUser,
) -> AppResult<Json<Vec<CategorySummary>>> {
    let now = chrono::Utc::now().naive_utc().date();
    let start_of_month = NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
        .expect("Valid date");

    let summaries = sqlx::query_as::<_, CategorySummary>(
        r#"
        SELECT
            categories.id as category_id,
            categories.name as category_name,
            categories.color as category_color,
            categories.icon as category_icon,
            COALESCE(SUM(expenses.amount), 0) as total_amount,
            COUNT(expenses.id)::BIGINT as expense_count
        FROM categories
        LEFT JOIN expenses ON categories.id = expenses.category_id
            AND expenses.expense_date >= $2
        WHERE categories.user_id = $1
        GROUP BY categories.id, categories.name, categories.color, categories.icon
        ORDER BY total_amount DESC
        "#,
    )
    .bind(user.user_id)
    .bind(start_of_month)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(summaries))
}
