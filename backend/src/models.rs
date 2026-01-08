use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// User model - demonstrates ownership and lifetimes
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    #[validate(length(min = 1, message = "Full name is required"))]
    pub full_name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            full_name: user.full_name,
            created_at: user.created_at,
        }
    }
}

// Category model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategory {
    #[validate(length(min = 1, max = 100, message = "Category name must be 1-100 characters"))]
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCategory {
    #[validate(length(min = 1, max = 100, message = "Category name must be 1-100 characters"))]
    pub name: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

// Expense model - demonstrates trait bounds and serialization
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Expense {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub amount: sqlx::types::Decimal,
    pub description: String,
    pub expense_date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ExpenseWithCategory {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub category_name: String,
    pub category_color: Option<String>,
    pub category_icon: Option<String>,
    pub amount: sqlx::types::Decimal,
    pub description: String,
    pub expense_date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateExpense {
    pub category_id: Uuid,
    #[validate(range(min = 0.01, message = "Amount must be greater than 0"))]
    pub amount: f64,
    #[validate(length(min = 1, message = "Description is required"))]
    pub description: String,
    pub expense_date: NaiveDate,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateExpense {
    pub category_id: Option<Uuid>,
    #[validate(range(min = 0.01, message = "Amount must be greater than 0"))]
    pub amount: Option<f64>,
    pub description: Option<String>,
    pub expense_date: Option<NaiveDate>,
}

// Query parameters for filtering
#[derive(Debug, Deserialize)]
pub struct ExpenseQuery {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub category_id: Option<Uuid>,
}

// Monthly summary - demonstrates aggregation
#[derive(Debug, Serialize, FromRow)]
pub struct MonthlySummary {
    pub month: String,
    pub year: i32,
    pub total_amount: sqlx::types::Decimal,
    pub expense_count: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct CategorySummary {
    pub category_id: Uuid,
    pub category_name: String,
    pub category_color: Option<String>,
    pub category_icon: Option<String>,
    pub total_amount: sqlx::types::Decimal,
    pub expense_count: i64,
}
