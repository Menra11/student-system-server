use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    pub course_id:    i32,
    pub course_name:  String,
    pub credit:       i32,
    pub teacher_id:   i32,
    pub classroom:    String,
    pub schedule:     String,
    pub description:  String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CourseResponse {
    pub success: bool,
    pub message: Option<String>,
    pub course: Option<Course>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoursesResponse {
    pub success: bool,
    pub message: Option<String>,
    pub courses: Option<Vec<Course>>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CoursesId {
    pub courses_id: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoursesSelectResponse {
    pub success: bool,
    pub message: Option<String>,
}


