use axum::{extract::State, http::StatusCode, Json};
use validator::Validate;

use crate::{
    auth::{create_jwt, hash_password, verify_password},
    error::{AppError, AppResult},
    models::{AuthResponse, CreateUser, LoginRequest, User, UserResponse},
    AppState,
};

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> AppResult<(StatusCode, Json<AuthResponse>)> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let email_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)"
    )
    .bind(&payload.email)
    .fetch_one(&state.pool)
    .await?;

    if email_exists {
        return Err(AppError::Validation("Email already registered".to_string()));
    }

    let password_hash = hash_password(&payload.password)?;

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, password_hash, full_name)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&payload.full_name)
    .fetch_one(&state.pool)
    .await?;

    let token = create_jwt(
        user.id,
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    )?;

    let response = AuthResponse {
        token,
        user: user.into(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::Authentication("Invalid credentials".to_string()))?;

    verify_password(&payload.password, &user.password_hash)?;

    let token = create_jwt(
        user.id,
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    )?;

    let response = AuthResponse {
        token,
        user: user.into(),
    };

    Ok(Json(response))
}

pub async fn get_current_user(
    State(state): State<AppState>,
    user: crate::auth::AuthUser,
) -> AppResult<Json<UserResponse>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user.user_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(user.into()))
}
