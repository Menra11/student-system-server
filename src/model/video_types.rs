use serde ::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    video_id:         u32,
    video_title:      String,
    video_description:String,
    video_url:        String,
    video_duration:   u32,
    teacher_name:     String,
    course_name:      String,
    completed:        Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Progress {
    progress_id:      u32,
    student_id:       u32,
    video_id:         u32,
    progress:         u32,
    completed:        bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoResponse {
    pub success       : bool,
    pub message       : Option<String>,
    pub video         : Option<Video>,
    pub progress      : Option<Progress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  VideosResponse {
    pub success       : bool,
    pub message       : Option<String>,
    pub videos        : Option<Vec<Video>>,
    pub progress      : Option<Vec<Progress>>,
}