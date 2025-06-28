use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginData {
  pub user_id: u32,
  pub password: String,
  pub user: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginDataRequest<T> {
  pub user_from: T
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
  pub success: bool,
  pub message: Option<String>,
  pub token: Option<String>,
  pub error_code: Option<String>,
}
