use serde ::{Deserialize, Serialize};
use chrono::NaiveDate;
#[derive(Debug, Serialize, Deserialize)]
pub struct Teacher {
    pub teacher_id    :i32,
    pub teacher_name  :String,
    pub gender        :Option<String>,
    pub title         :Option<String>,
    pub birth_date    :Option<NaiveDate>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentsInfo {
    pub student_id    :i32,
    pub student_name  :String,
    pub class_name    :String,
    pub course_name   :String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentsInfoResponse {
    pub success       : bool,
    pub message       : Option<String>,
    pub students_info : Option<Vec<StudentsInfo>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoursesInfo {
    pub course_id:    i32,
    pub course_name:  String,
    pub credit:       i32,
    pub classroom:    String,
    pub schedule:     String,
    pub description:  String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoursesInfoResponse {
    pub success       : bool,
    pub message       : Option<String>,
    pub courses_info : Option<Vec<CoursesInfo>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideosInfo {
    pub student_id:    Option<i32>,
    pub student_name:  Option<String>,
    pub course_id:     Option<i32>,
    pub course_name:   Option<String>,
    pub video_title:   Option<String>,
    pub video_duration:Option<i32>,
    pub progress:      Option<f32>,
    pub completed:     Option<bool>,
    pub score:         Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideosInfoResponse {
    pub success       : bool,
    pub message       : Option<String>,
    pub videos_info   : Option<Vec<VideosInfo>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseVideos {
    pub video_id:           i32,
    pub video_title:        Option<String>,
    pub video_description:  Option<String>,
    pub video_url:          Option<String>,
    pub video_duration:     Option<i32>,
    pub course_id:          i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseVideosResponse {
    pub success       : bool,
    pub message       : Option<String>,
    pub course_videos   : Option<Vec<CourseVideos>>,
}


