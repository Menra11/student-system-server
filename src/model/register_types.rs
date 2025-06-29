use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct RegisterData {
    pub student_id:     Option<u32>,
    pub student_name:   Option<String>,
    pub gender:         Option<String>,
    pub birth_date:     Option<String>,
    pub class_id:       Option<u32>,
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
