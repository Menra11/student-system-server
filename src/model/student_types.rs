use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    pub student_id: i64,
    pub student_name: String,
    pub gender:String,
    pub birth_date: String,
    pub class_id: i64,
    pub class_name: String,
    pub phone: String,
    pub email: String,
}
