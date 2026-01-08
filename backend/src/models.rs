//! Data models for the expense tracker application.
//!
//! This module defines all the data structures used throughout the application,
//! including database models, request/response types, and query parameters.
//!
//! # Rust Concepts Demonstrated
//!
//! - **Ownership**: Structs own their String fields, demonstrating owned data
//! - **Serialization**: Extensive use of serde for JSON conversion
//! - **Traits**: Multiple derive macros showing trait composition
//! - **Validation**: Input validation using the validator crate
//! - **Type Safety**: NewType pattern with UUIDs and specific types

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// ============================================================================
// User Models
// ============================================================================

/// Represents a user in the database.
///
/// This struct demonstrates **ownership** and **lifetimes** in Rust:
/// - All fields are owned (String, not &str)
/// - Derives FromRow for automatic database mapping
/// - Derives Serialize for JSON responses (but NOT Deserialize - password hash should never be deserialized from client input)
///
/// # Database Schema
/// ```sql
/// CREATE TABLE users (
///     id UUID PRIMARY KEY,
///     email VARCHAR(255) UNIQUE NOT NULL,
///     password_hash VARCHAR(255) NOT NULL,
///     full_name VARCHAR(255) NOT NULL,
///     created_at TIMESTAMPTZ NOT NULL,
///     updated_at TIMESTAMPTZ NOT NULL
/// );
/// ```
///
/// # Example
/// ```rust,ignore
/// let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
///     .bind(user_id)
///     .fetch_one(&pool)
///     .await?;
/// ```
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    /// Unique identifier for the user (UUID v4)
    pub id: Uuid,
    /// User's email address (must be unique, validated on input)
    pub email: String,
    /// Argon2 hashed password (never sent to client)
    pub password_hash: String,
    /// User's display name
    pub full_name: String,
    /// Timestamp when the user was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the user was last updated
    pub updated_at: DateTime<Utc>,
}

/// Request body for user registration.
///
/// Demonstrates **validation** using the validator crate.
/// Each field has validation rules that are checked before processing.
///
/// # Validation Rules
/// - `email`: Must be a valid email format
/// - `password`: Minimum 8 characters
/// - `full_name`: At least 1 character (non-empty)
///
/// # Example
/// ```json
/// {
///   "email": "user@example.com",
///   "password": "securepassword123",
///   "full_name": "John Doe"
/// }
/// ```
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    /// User's email address (validated for proper email format)
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    /// Plain text password (will be hashed with Argon2 before storage)
    /// Minimum 8 characters for basic security
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    /// User's full name (cannot be empty)
    #[validate(length(min = 1, message = "Full name is required"))]
    pub full_name: String,
}

/// Request body for user login.
///
/// Simpler than CreateUser as we only need credentials.
/// Password is validated on the backend by comparing hashes.
///
/// # Example
/// ```json
/// {
///   "email": "user@example.com",
///   "password": "securepassword123"
/// }
/// ```
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    /// User's email address
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    /// Plain text password (will be verified against stored hash)
    pub password: String,
}

/// Response returned after successful authentication.
///
/// Contains both a JWT token and user information.
/// The token should be stored by the client and sent in subsequent requests.
///
/// # Example Response
/// ```json
/// {
///   "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
///   "user": {
///     "id": "123e4567-e89b-12d3-a456-426614174000",
///     "email": "user@example.com",
///     "full_name": "John Doe",
///     "created_at": "2024-01-01T00:00:00Z"
///   }
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    /// JWT token for authentication (include in Authorization header)
    pub token: String,
    /// User information (without sensitive data like password hash)
    pub user: UserResponse,
}

/// User information sent to clients.
///
/// This is a sanitized version of the User model that excludes
/// sensitive information like password hashes.
///
/// Demonstrates the **From trait** for type conversion.
#[derive(Debug, Serialize)]
pub struct UserResponse {
    /// User's unique identifier
    pub id: Uuid,
    /// User's email address
    pub email: String,
    /// User's full name
    pub full_name: String,
    /// Account creation timestamp
    pub created_at: DateTime<Utc>,
}

/// Converts a User model into a UserResponse (safe for sending to clients).
///
/// This implementation demonstrates Rust's **From trait** for clean type conversions.
/// The password_hash field is automatically excluded, preventing accidental exposure.
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

// ============================================================================
// Category Models
// ============================================================================

/// Represents an expense category in the database.
///
/// Categories help users organize their expenses (e.g., Food, Transportation, Entertainment).
/// Each user has their own set of categories, plus default categories created on registration.
///
/// # Database Schema
/// ```sql
/// CREATE TABLE categories (
///     id UUID PRIMARY KEY,
///     user_id UUID NOT NULL REFERENCES users(id),
///     name VARCHAR(100) NOT NULL,
///     color VARCHAR(7),  -- Hex color code like "#FF6B6B"
///     icon VARCHAR(50),  -- Emoji or icon identifier like "🍔"
///     created_at TIMESTAMPTZ NOT NULL,
///     UNIQUE(user_id, name)
/// );
/// ```
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Category {
    /// Unique identifier for the category
    pub id: Uuid,
    /// ID of the user who owns this category
    pub user_id: Uuid,
    /// Category name (unique per user)
    pub name: String,
    /// Optional hex color code for UI display (e.g., "#FF6B6B")
    pub color: Option<String>,
    /// Optional emoji or icon identifier (e.g., "🍔")
    pub icon: Option<String>,
    /// Timestamp when the category was created
    pub created_at: DateTime<Utc>,
}

/// Request body for creating a new category.
///
/// # Example
/// ```json
/// {
///   "name": "Groceries",
///   "color": "#4ECDC4",
///   "icon": "🛒"
/// }
/// ```
#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategory {
    /// Category name (1-100 characters, must be unique for the user)
    #[validate(length(min = 1, max = 100, message = "Category name must be 1-100 characters"))]
    pub name: String,
    /// Optional hex color code for visual identification
    pub color: Option<String>,
    /// Optional emoji or icon for visual identification
    pub icon: Option<String>,
}

/// Request body for updating an existing category.
///
/// All fields are optional - only provided fields will be updated.
/// Demonstrates Rust's **Option type** for partial updates.
///
/// # Example (only updating name)
/// ```json
/// {
///   "name": "Grocery Shopping"
/// }
/// ```
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCategory {
    /// New category name (optional)
    #[validate(length(min = 1, max = 100, message = "Category name must be 1-100 characters"))]
    pub name: Option<String>,
    /// New color (optional)
    pub color: Option<String>,
    /// New icon (optional)
    pub icon: Option<String>,
}

// ============================================================================
// Expense Models
// ============================================================================

/// Represents an expense record in the database.
///
/// This is the core model of the application, tracking individual expenses.
/// Demonstrates **trait bounds** with multiple derive macros and **serialization**.
///
/// # Database Schema
/// ```sql
/// CREATE TABLE expenses (
///     id UUID PRIMARY KEY,
///     user_id UUID NOT NULL REFERENCES users(id),
///     category_id UUID NOT NULL REFERENCES categories(id),
///     amount DECIMAL(12, 2) NOT NULL CHECK (amount > 0),
///     description TEXT NOT NULL,
///     expense_date DATE NOT NULL,
///     created_at TIMESTAMPTZ NOT NULL,
///     updated_at TIMESTAMPTZ NOT NULL
/// );
/// ```
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Expense {
    /// Unique identifier for the expense
    pub id: Uuid,
    /// ID of the user who created this expense
    pub user_id: Uuid,
    /// ID of the category this expense belongs to
    pub category_id: Uuid,
    /// Amount spent (stored as DECIMAL for precise financial calculations)
    pub amount: Decimal,
    /// Description of what was purchased/paid for
    pub description: String,
    /// Date when the expense occurred (not necessarily when it was recorded)
    pub expense_date: NaiveDate,
    /// Timestamp when the expense record was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the expense record was last updated
    pub updated_at: DateTime<Utc>,
}

/// Expense data joined with category information.
///
/// This struct is used for API responses to avoid N+1 query problems.
/// Instead of fetching expenses then fetching each category separately,
/// we join them in a single query for better performance.
///
/// # SQL Query Example
/// ```sql
/// SELECT
///     expenses.*,
///     categories.name as category_name,
///     categories.color as category_color,
///     categories.icon as category_icon
/// FROM expenses
/// JOIN categories ON expenses.category_id = categories.id
/// WHERE expenses.user_id = $1
/// ```
#[derive(Debug, Serialize, FromRow)]
pub struct ExpenseWithCategory {
    /// Expense unique identifier
    pub id: Uuid,
    /// User who owns this expense
    pub user_id: Uuid,
    /// Category this expense belongs to
    pub category_id: Uuid,
    /// Category name (from joined table)
    pub category_name: String,
    /// Category color (from joined table)
    pub category_color: Option<String>,
    /// Category icon (from joined table)
    pub category_icon: Option<String>,
    /// Amount spent
    pub amount: Decimal,
    /// Description of the expense
    pub description: String,
    /// Date of the expense
    pub expense_date: NaiveDate,
    /// When this record was created
    pub created_at: DateTime<Utc>,
    /// When this record was last updated
    pub updated_at: DateTime<Utc>,
}

/// Request body for creating a new expense.
///
/// # Example
/// ```json
/// {
///   "category_id": "123e4567-e89b-12d3-a456-426614174000",
///   "amount": 42.50,
///   "description": "Lunch at restaurant",
///   "expense_date": "2024-01-15"
/// }
/// ```
#[derive(Debug, Deserialize, Validate)]
pub struct CreateExpense {
    /// ID of the category for this expense (must belong to the user)
    pub category_id: Uuid,

    /// Amount spent (must be greater than 0)
    /// Stored as f64 for JSON compatibility, converted to Decimal for database
    #[validate(range(min = 0.01, message = "Amount must be greater than 0"))]
    pub amount: f64,

    /// Description of the expense (required, at least 1 character)
    #[validate(length(min = 1, message = "Description is required"))]
    pub description: String,

    /// Date when the expense occurred (ISO 8601 format: YYYY-MM-DD)
    pub expense_date: NaiveDate,
}

/// Request body for updating an existing expense.
///
/// All fields are optional for partial updates.
/// Demonstrates Rust's **Option type** for flexible APIs.
///
/// # Example (only updating amount)
/// ```json
/// {
///   "amount": 45.00
/// }
/// ```
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateExpense {
    /// New category (optional)
    pub category_id: Option<Uuid>,

    /// New amount (optional, must be > 0 if provided)
    #[validate(range(min = 0.01, message = "Amount must be greater than 0"))]
    pub amount: Option<f64>,

    /// New description (optional)
    pub description: Option<String>,

    /// New date (optional)
    pub expense_date: Option<NaiveDate>,
}

// ============================================================================
// Query Models
// ============================================================================

/// Query parameters for filtering expenses.
///
/// Used as URL query parameters, e.g.:
/// `/api/expenses?start_date=2024-01-01&end_date=2024-01-31&category_id=...`
///
/// All fields are optional, allowing flexible filtering:
/// - No params: Return all expenses
/// - Only start_date: Expenses from that date onwards
/// - start_date + end_date: Expenses in date range
/// - category_id: Only expenses in that category
/// - Combine all: Expenses in category within date range
///
/// # Example URL
/// ```
/// GET /api/expenses?start_date=2024-01-01&end_date=2024-01-31&category_id=123e4567-e89b-12d3-a456-426614174000
/// ```
#[derive(Debug, Deserialize)]
pub struct ExpenseQuery {
    /// Filter expenses from this date onwards (inclusive)
    pub start_date: Option<NaiveDate>,
    /// Filter expenses up to this date (inclusive)
    pub end_date: Option<NaiveDate>,
    /// Filter expenses by category
    pub category_id: Option<Uuid>,
}

// ============================================================================
// Summary Models
// ============================================================================

/// Monthly expense summary for analytics.
///
/// Demonstrates **database aggregation** with GROUP BY queries.
/// Used for displaying monthly spending trends.
///
/// # SQL Query Example
/// ```sql
/// SELECT
///     TO_CHAR(expense_date, 'Month') as month,
///     EXTRACT(YEAR FROM expense_date)::INTEGER as year,
///     SUM(amount) as total_amount,
///     COUNT(*)::BIGINT as expense_count
/// FROM expenses
/// WHERE user_id = $1
/// GROUP BY month, year
/// ORDER BY year DESC, MIN(expense_date) DESC
/// ```
///
/// # Example Response
/// ```json
/// {
///   "month": "January",
///   "year": 2024,
///   "total_amount": 1523.45,
///   "expense_count": 42
/// }
/// ```
#[derive(Debug, Serialize, FromRow)]
pub struct MonthlySummary {
    /// Month name (e.g., "January", "February")
    pub month: String,
    /// Year as integer
    pub year: i32,
    /// Total amount spent in this month
    pub total_amount: Decimal,
    /// Number of expenses in this month
    pub expense_count: i64,
}

/// Category-wise expense summary.
///
/// Shows spending breakdown by category for the current month.
/// Useful for visualizing where money is being spent.
///
/// # SQL Query Example
/// ```sql
/// SELECT
///     categories.id as category_id,
///     categories.name as category_name,
///     categories.color as category_color,
///     categories.icon as category_icon,
///     COALESCE(SUM(expenses.amount), 0) as total_amount,
///     COUNT(expenses.id)::BIGINT as expense_count
/// FROM categories
/// LEFT JOIN expenses ON categories.id = expenses.category_id
///     AND expenses.expense_date >= $2
/// WHERE categories.user_id = $1
/// GROUP BY categories.id, categories.name, categories.color, categories.icon
/// ```
///
/// # Example Response
/// ```json
/// {
///   "category_id": "123e4567-e89b-12d3-a456-426614174000",
///   "category_name": "Food & Dining",
///   "category_color": "#FF6B6B",
///   "category_icon": "🍔",
///   "total_amount": 450.25,
///   "expense_count": 15
/// }
/// ```
#[derive(Debug, Serialize, FromRow)]
pub struct CategorySummary {
    /// Category unique identifier
    pub category_id: Uuid,
    /// Category name
    pub category_name: String,
    /// Category color for UI
    pub category_color: Option<String>,
    /// Category icon for UI
    pub category_icon: Option<String>,
    /// Total amount spent in this category
    pub total_amount: Decimal,
    /// Number of expenses in this category
    pub expense_count: i64,
}
