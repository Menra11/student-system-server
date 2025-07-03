use serde ::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    pub video_id:         u32,
    pub video_title:      Option<String>,
    pub video_description:Option<String>,
    pub video_url:        Option<String>,
    pub video_duration:   Option<u32>,
    pub teacher_name:     Option<String>,
    pub course_name:      Option<String>,
    pub course_id:        Option<u32>,
    pub completed:        Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllVideo {
    pub video_title:      Option<String>,
    pub video_description:Option<String>,
    pub video_url:        Option<String>,
    pub video_duration:   Option<u32>,
    pub course_id:        u32,
}
    
#[derive(Debug, Serialize, Deserialize)]
pub struct Progress {
    pub progress_id:      u32,
    pub student_id:       u32,
    pub video_id:         u32,
    pub progress:         f32,
    pub completed:        bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressRequest { 
    pub completed:        u32,
    pub progress:         f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoResponse {
    pub success          : bool,
    pub message          : Option<String>,
    pub video            : Option<Video>,
    pub progress         : Option<Progress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  VideosResponse {
    pub success         : bool,
    pub message         : Option<String>,
    pub videos          : Option<Vec<Video>>,
    pub progresses        : Option<Vec<Progress>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  VideoTitle {
    pub title         : String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  VideoTitleResponse {
    pub success         : bool,
    pub message         : Option<String>,
    pub file_name       :Option< String>,
}

