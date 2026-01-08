use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use rust_decimal::Decimal;
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::AuthUser,
    error::{AppError, AppResult},
    models::{CreateExpense, ExpenseQuery, ExpenseWithCategory, UpdateExpense},
    AppState,
};

pub async fn create_expense(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateExpense>,
) -> AppResult<(StatusCode, Json<ExpenseWithCategory>)> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let category_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM categories WHERE id = $1 AND user_id = $2)"
    )
    .bind(payload.category_id)
    .bind(user.user_id)
    .fetch_one(&state.pool)
    .await?;

    if !category_exists {
        return Err(AppError::NotFound("Category not found".to_string()));
    }

    let amount = Decimal::try_from(payload.amount)
        .map_err(|_| AppError::Validation("Invalid amount".to_string()))?;

    let expense = sqlx::query_as::<_, ExpenseWithCategory>(
        r#"
        INSERT INTO expenses (user_id, category_id, amount, description, expense_date)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING
            expenses.id,
            expenses.user_id,
            expenses.category_id,
            categories.name as category_name,
            categories.color as category_color,
            categories.icon as category_icon,
            expenses.amount,
            expenses.description,
            expenses.expense_date,
            expenses.created_at,
            expenses.updated_at
        FROM expenses
        JOIN categories ON expenses.category_id = categories.id
        WHERE expenses.id = expenses.id
        "#,
    )
    .bind(user.user_id)
    .bind(payload.category_id)
    .bind(amount)
    .bind(&payload.description)
    .bind(payload.expense_date)
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(expense)))
}

pub async fn list_expenses(
    State(state): State<AppState>,
    user: AuthUser,
    Query(query): Query<ExpenseQuery>,
) -> AppResult<Json<Vec<ExpenseWithCategory>>> {
    let mut sql = String::from(
        r#"
        SELECT
            expenses.id,
            expenses.user_id,
            expenses.category_id,
            categories.name as category_name,
            categories.color as category_color,
            categories.icon as category_icon,
            expenses.amount,
            expenses.description,
            expenses.expense_date,
            expenses.created_at,
            expenses.updated_at
        FROM expenses
        JOIN categories ON expenses.category_id = categories.id
        WHERE expenses.user_id = $1
        "#,
    );

    let mut param_count = 1;

    if query.start_date.is_some() {
        param_count += 1;
        sql.push_str(&format!(" AND expenses.expense_date >= ${}", param_count));
    }

    if query.end_date.is_some() {
        param_count += 1;
        sql.push_str(&format!(" AND expenses.expense_date <= ${}", param_count));
    }

    if query.category_id.is_some() {
        param_count += 1;
        sql.push_str(&format!(" AND expenses.category_id = ${}", param_count));
    }

    sql.push_str(" ORDER BY expenses.expense_date DESC, expenses.created_at DESC");

    let mut query_builder = sqlx::query_as::<_, ExpenseWithCategory>(&sql).bind(user.user_id);

    if let Some(start_date) = query.start_date {
        query_builder = query_builder.bind(start_date);
    }

    if let Some(end_date) = query.end_date {
        query_builder = query_builder.bind(end_date);
    }

    if let Some(category_id) = query.category_id {
        query_builder = query_builder.bind(category_id);
    }

    let expenses = query_builder.fetch_all(&state.pool).await?;

    Ok(Json(expenses))
}

pub async fn get_expense(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<ExpenseWithCategory>> {
    let expense = sqlx::query_as::<_, ExpenseWithCategory>(
        r#"
        SELECT
            expenses.id,
            expenses.user_id,
            expenses.category_id,
            categories.name as category_name,
            categories.color as category_color,
            categories.icon as category_icon,
            expenses.amount,
            expenses.description,
            expenses.expense_date,
            expenses.created_at,
            expenses.updated_at
        FROM expenses
        JOIN categories ON expenses.category_id = categories.id
        WHERE expenses.id = $1 AND expenses.user_id = $2
        "#,
    )
    .bind(id)
    .bind(user.user_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Expense not found".to_string()))?;

    Ok(Json(expense))
}

pub async fn update_expense(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateExpense>,
) -> AppResult<Json<ExpenseWithCategory>> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let expense_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM expenses WHERE id = $1 AND user_id = $2)"
    )
    .bind(id)
    .bind(user.user_id)
    .fetch_one(&state.pool)
    .await?;

    if !expense_exists {
        return Err(AppError::NotFound("Expense not found".to_string()));
    }

    if let Some(category_id) = payload.category_id {
        let category_exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM categories WHERE id = $1 AND user_id = $2)"
        )
        .bind(category_id)
        .bind(user.user_id)
        .fetch_one(&state.pool)
        .await?;

        if !category_exists {
            return Err(AppError::NotFound("Category not found".to_string()));
        }
    }

    let mut sql = String::from("UPDATE expenses SET updated_at = NOW()");
    let mut update_fields = Vec::new();

    if let Some(category_id) = payload.category_id {
        update_fields.push(format!("category_id = '{}'", category_id));
    }

    if let Some(amount) = payload.amount {
        let decimal_amount = Decimal::try_from(amount)
            .map_err(|_| AppError::Validation("Invalid amount".to_string()))?;
        update_fields.push(format!("amount = {}", decimal_amount));
    }

    if let Some(description) = &payload.description {
        update_fields.push(format!("description = '{}'", description.replace("'", "''")));
    }

    if let Some(expense_date) = payload.expense_date {
        update_fields.push(format!("expense_date = '{}'", expense_date));
    }

    if !update_fields.is_empty() {
        sql.push_str(", ");
        sql.push_str(&update_fields.join(", "));
    }

    sql.push_str(&format!(
        " WHERE id = '{}' AND user_id = '{}'",
        id, user.user_id
    ));

    sqlx::query(&sql).execute(&state.pool).await?;

    let updated_expense = sqlx::query_as::<_, ExpenseWithCategory>(
        r#"
        SELECT
            expenses.id,
            expenses.user_id,
            expenses.category_id,
            categories.name as category_name,
            categories.color as category_color,
            categories.icon as category_icon,
            expenses.amount,
            expenses.description,
            expenses.expense_date,
            expenses.created_at,
            expenses.updated_at
        FROM expenses
        JOIN categories ON expenses.category_id = categories.id
        WHERE expenses.id = $1
        "#,
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(updated_expense))
}

pub async fn delete_expense(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let result = sqlx::query("DELETE FROM expenses WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user.user_id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Expense not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
