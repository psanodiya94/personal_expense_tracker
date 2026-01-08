use chrono::{NaiveDate, DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub full_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateCategory {
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub category_name: String,
    pub category_color: Option<String>,
    pub category_icon: Option<String>,
    pub amount: f64,
    pub description: String,
    pub expense_date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateExpense {
    pub category_id: Uuid,
    pub amount: f64,
    pub description: String,
    pub expense_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateExpense {
    pub category_id: Option<Uuid>,
    pub amount: Option<f64>,
    pub description: Option<String>,
    pub expense_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlySummary {
    pub month: String,
    pub year: i32,
    pub total_amount: f64,
    pub expense_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySummary {
    pub category_id: Uuid,
    pub category_name: String,
    pub category_color: Option<String>,
    pub category_icon: Option<String>,
    pub total_amount: f64,
    pub expense_count: i64,
}
