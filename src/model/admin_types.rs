use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Admin {
    pub admin_id:    i32,
    pub admin_name:  String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AdminResponse {
    pub success: bool,
    pub message: Option<String>,
    pub admin: Option<Admin>,
}