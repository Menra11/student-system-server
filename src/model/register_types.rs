use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize)]

pub struct RegisterData {
    pub student_id:     Option<i32>,
    pub student_name:   Option<String>,
    pub gender:         Option<String>,
    pub birth_date:     Option<NaiveDate>,
    pub class_id:       Option<i32>,
    pub phone:          Option<String>,
    pub email:          Option<String>,
    pub password:       Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterDataRequest<T> {
  pub user_from: T
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
  pub success: bool,
  pub message: Option<String>,
}
