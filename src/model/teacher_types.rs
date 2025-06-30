use serde ::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Teacher {
    pub teacher_id    :u32,
    pub teacher_name  :String,
    pub gender        :Option<String>,
    pub title         :Option<String>,
    pub birth_date    :Option<String>,
    pub phone         :Option<String>,
    pub email         :Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherResponse {
    pub success       : bool,
    pub message       : Option<String>,
    pub teacher       : Option<Teacher>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  TeachersResponse {
    pub success       : bool,
    pub message       : Option<String>,
    pub teachers      : Option<Vec<Teacher>>,
}