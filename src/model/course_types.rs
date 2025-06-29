use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    pub course_id:    u32,
    pub course_name:  String,
    pub credit:       String,
    pub teacher_id:   u32,
    pub classroom:    String,
    pub schedule:     String,
    pub description:  String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CourseResponse {
    pub success: bool,
    pub message: Option<String>,
    pub courses: Option<Vec<Course>>,
}



