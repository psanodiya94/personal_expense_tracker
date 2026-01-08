use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    handlers::{categories, expenses, summaries, users},
    AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(|| async { "OK" }))
        // Auth routes (public)
        .route("/api/auth/register", post(users::register))
        .route("/api/auth/login", post(users::login))
        // User routes (protected)
        .route("/api/users/me", get(users::get_current_user))
        // Category routes (protected)
        .route("/api/categories", post(categories::create_category))
        .route("/api/categories", get(categories::list_categories))
        .route("/api/categories/:id", get(categories::get_category))
        .route("/api/categories/:id", put(categories::update_category))
        .route("/api/categories/:id", delete(categories::delete_category))
        // Expense routes (protected)
        .route("/api/expenses", post(expenses::create_expense))
        .route("/api/expenses", get(expenses::list_expenses))
        .route("/api/expenses/:id", get(expenses::get_expense))
        .route("/api/expenses/:id", put(expenses::update_expense))
        .route("/api/expenses/:id", delete(expenses::delete_expense))
        // Summary routes (protected)
        .route("/api/summaries/monthly", get(summaries::get_monthly_summary))
        .route("/api/summaries/categories", get(summaries::get_category_summary))
        .with_state(state)
}
