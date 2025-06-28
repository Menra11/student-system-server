use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginData {
  pub user_id: Option<u32>,
  pub password: Option<String>,
  pub user: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StudentLoginData {
    pub student_id: i64,
    pub student_name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherLoginData {
    pub teacher_id: i64,
    pub teacher_name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminLoginData {
    pub admin_id: i64,
    pub admin_name: String,
    pub password: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub userid: u32,
    pub usertype: String,
    pub exp: i64,
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
