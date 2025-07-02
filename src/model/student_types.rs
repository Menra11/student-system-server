use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    pub student_id: u32,
    pub student_name: String,
    pub gender: String,
    pub birth_date: String,
    pub class_id: u32,
    pub class_name: String,
    pub phone: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Score {
    pub score_id:       u32,
    pub course_name:    String,
    pub credit:         u32,
    pub teacher_name:   String,
    pub score:          Option<f32>,
    pub semester:       String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentResponse {
    pub success: bool,
    pub message: Option<String>,
    pub student: Option<Student>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreResponse {
    pub success: bool,
    pub message: Option<String>,
    pub scores: Option<Vec<Score>>,
}


