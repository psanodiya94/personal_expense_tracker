use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web_sys::window;

use crate::models::*;

const API_BASE: &str = "http://localhost:3000/api";

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn get_token() -> Option<String> {
    window()?
        .local_storage()
        .ok()??
        .get_item("token")
        .ok()?
}

pub fn set_token(token: &str) {
    if let Some(storage) = window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
    {
        let _ = storage.set_item("token", token);
    }
}

pub fn clear_token() {
    if let Some(storage) = window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
    {
        let _ = storage.remove_item("token");
    }
}

pub async fn register(req: RegisterRequest) -> Result<AuthResponse, String> {
    let response = Request::post(&format!("{}/auth/register", API_BASE))
        .json(&req)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        let auth = response.json::<AuthResponse>().await
            .map_err(|e| e.to_string())?;
        set_token(&auth.token);
        Ok(auth)
    } else {
        let error = response.json::<ErrorResponse>().await
            .map_err(|e| e.to_string())?;
        Err(error.error)
    }
}

pub async fn login(req: LoginRequest) -> Result<AuthResponse, String> {
    let response = Request::post(&format!("{}/auth/login", API_BASE))
        .json(&req)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        let auth = response.json::<AuthResponse>().await
            .map_err(|e| e.to_string())?;
        set_token(&auth.token);
        Ok(auth)
    } else {
        let error = response.json::<ErrorResponse>().await
            .map_err(|e| e.to_string())?;
        Err(error.error)
    }
}

pub async fn get_current_user() -> Result<User, String> {
    let token = get_token().ok_or("No token found")?;

    let response = Request::get(&format!("{}/users/me", API_BASE))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        response.json::<User>().await.map_err(|e| e.to_string())
    } else {
        let error = response.json::<ErrorResponse>().await
            .map_err(|e| e.to_string())?;
        Err(error.error)
    }
}

pub async fn list_categories() -> Result<Vec<Category>, String> {
    let token = get_token().ok_or("No token found")?;

    let response = Request::get(&format!("{}/categories", API_BASE))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        response.json::<Vec<Category>>().await.map_err(|e| e.to_string())
    } else {
        let error = response.json::<ErrorResponse>().await
            .map_err(|e| e.to_string())?;
        Err(error.error)
    }
}

pub async fn create_category(req: CreateCategory) -> Result<Category, String> {
    let token = get_token().ok_or("No token found")?;

    let response = Request::post(&format!("{}/categories", API_BASE))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&req)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        response.json::<Category>().await.map_err(|e| e.to_string())
    } else {
        let error = response.json::<ErrorResponse>().await
            .map_err(|e| e.to_string())?;
        Err(error.error)
    }
}

pub async fn list_expenses(
    start_date: Option<String>,
    end_date: Option<String>,
    category_id: Option<Uuid>,
) -> Result<Vec<Expense>, String> {
    let token = get_token().ok_or("No token found")?;

    let mut url = format!("{}/expenses", API_BASE);
    let mut params = Vec::new();

    if let Some(start) = start_date {
        params.push(format!("start_date={}", start));
    }
    if let Some(end) = end_date {
        params.push(format!("end_date={}", end));
    }
    if let Some(cat_id) = category_id {
        params.push(format!("category_id={}", cat_id));
    }

    if !params.is_empty() {
        url.push('?');
        url.push_str(&params.join("&"));
    }

    let response = Request::get(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        response.json::<Vec<Expense>>().await.map_err(|e| e.to_string())
    } else {
        let error = response.json::<ErrorResponse>().await
            .map_err(|e| e.to_string())?;
        Err(error.error)
    }
}

pub async fn create_expense(req: CreateExpense) -> Result<Expense, String> {
    let token = get_token().ok_or("No token found")?;

    let response = Request::post(&format!("{}/expenses", API_BASE))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&req)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        response.json::<Expense>().await.map_err(|e| e.to_string())
    } else {
        let error = response.json::<ErrorResponse>().await
            .map_err(|e| e.to_string())?;
        Err(error.error)
    }
}

pub async fn delete_expense(id: Uuid) -> Result<(), String> {
    let token = get_token().ok_or("No token found")?;

    let response = Request::delete(&format!("{}/expenses/{}", API_BASE, id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        Ok(())
    } else {
        let error = response.json::<ErrorResponse>().await
            .map_err(|e| e.to_string())?;
        Err(error.error)
    }
}

pub async fn get_monthly_summary() -> Result<Vec<MonthlySummary>, String> {
    let token = get_token().ok_or("No token found")?;

    let response = Request::get(&format!("{}/summaries/monthly", API_BASE))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        response.json::<Vec<MonthlySummary>>().await.map_err(|e| e.to_string())
    } else {
        let error = response.json::<ErrorResponse>().await
            .map_err(|e| e.to_string())?;
        Err(error.error)
    }
}

pub async fn get_category_summary() -> Result<Vec<CategorySummary>, String> {
    let token = get_token().ok_or("No token found")?;

    let response = Request::get(&format!("{}/summaries/categories", API_BASE))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        response.json::<Vec<CategorySummary>>().await.map_err(|e| e.to_string())
    } else {
        let error = response.json::<ErrorResponse>().await
            .map_err(|e| e.to_string())?;
        Err(error.error)
    }
}
