use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::AuthUser,
    error::{AppError, AppResult},
    models::{Category, CreateCategory, UpdateCategory},
    AppState,
};

pub async fn create_category(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateCategory>,
) -> AppResult<(StatusCode, Json<Category>)> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let name_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM categories WHERE user_id = $1 AND name = $2)"
    )
    .bind(user.user_id)
    .bind(&payload.name)
    .fetch_one(&state.pool)
    .await?;

    if name_exists {
        return Err(AppError::Validation(
            "Category name already exists".to_string(),
        ));
    }

    let category = sqlx::query_as::<_, Category>(
        r#"
        INSERT INTO categories (user_id, name, color, icon)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(user.user_id)
    .bind(&payload.name)
    .bind(&payload.color)
    .bind(&payload.icon)
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(category)))
}

pub async fn list_categories(
    State(state): State<AppState>,
    user: AuthUser,
) -> AppResult<Json<Vec<Category>>> {
    let categories = sqlx::query_as::<_, Category>(
        "SELECT * FROM categories WHERE user_id = $1 ORDER BY name"
    )
    .bind(user.user_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(categories))
}

pub async fn get_category(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Category>> {
    let category = sqlx::query_as::<_, Category>(
        "SELECT * FROM categories WHERE id = $1 AND user_id = $2"
    )
    .bind(id)
    .bind(user.user_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;

    Ok(Json(category))
}

pub async fn update_category(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCategory>,
) -> AppResult<Json<Category>> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let category_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM categories WHERE id = $1 AND user_id = $2)"
    )
    .bind(id)
    .bind(user.user_id)
    .fetch_one(&state.pool)
    .await?;

    if !category_exists {
        return Err(AppError::NotFound("Category not found".to_string()));
    }

    if let Some(ref name) = payload.name {
        let name_exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM categories WHERE user_id = $1 AND name = $2 AND id != $3)"
        )
        .bind(user.user_id)
        .bind(name)
        .bind(id)
        .fetch_one(&state.pool)
        .await?;

        if name_exists {
            return Err(AppError::Validation(
                "Category name already exists".to_string(),
            ));
        }
    }

    let mut sql = String::from("UPDATE categories SET ");
    let mut updates = Vec::new();

    if let Some(name) = &payload.name {
        updates.push(format!("name = '{}'", name.replace("'", "''")));
    }

    if let Some(color) = &payload.color {
        updates.push(format!("color = '{}'", color.replace("'", "''")));
    }

    if let Some(icon) = &payload.icon {
        updates.push(format!("icon = '{}'", icon.replace("'", "''")));
    }

    if updates.is_empty() {
        return Err(AppError::Validation("No fields to update".to_string()));
    }

    sql.push_str(&updates.join(", "));
    sql.push_str(&format!(" WHERE id = '{}' AND user_id = '{}'", id, user.user_id));

    sqlx::query(&sql).execute(&state.pool).await?;

    let updated_category = sqlx::query_as::<_, Category>(
        "SELECT * FROM categories WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(updated_category))
}

pub async fn delete_category(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let has_expenses = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM expenses WHERE category_id = $1)"
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await?;

    if has_expenses {
        return Err(AppError::Validation(
            "Cannot delete category with existing expenses".to_string(),
        ));
    }

    let result = sqlx::query("DELETE FROM categories WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user.user_id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Category not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
