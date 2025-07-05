use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    pub student_id:     i32,
    pub student_name:   String,
    pub gender:         String,
    pub birth_date:     NaiveDate,
    pub class_id:       i32,
    pub class_name:     String,
    pub phone:          String,
    pub email:          String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Score {
    pub score_id: i32,
    pub course_name: String,
    pub credit: i32,
    pub teacher_name: String,
    pub score: Option<f32>,
    pub semester: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreRequest {
    pub student_id:    i32,
    pub course_id:  i32,
    pub score:      f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentResponse {
    pub success: bool,
    pub message: Option<String>,
    pub student: Option<Student>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentsResponse {
    pub success: bool,
    pub message: Option<String>,
    pub students: Option<Vec<Student>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreResponse {
    pub success: bool,
    pub message: Option<String>,
    pub scores: Option<Vec<Score>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class {
    pub class_id:   i64,
    pub class_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassesResponse {
    pub success: bool,
    pub message: Option<String>,
    pub classes: Option<Vec<Class>>,
}

