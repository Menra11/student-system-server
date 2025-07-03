use crate::model::*;
use mysql::{prelude::*, *};
use salvo::prelude::*;

#[handler]
pub async fn get_videos_info(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let id = req.param::<i64>("id").unwrap();

    let video_query = "select sp.student_id,s.student_name,c.course_id,c.course_name,v.video_title,v.video_duration,sp.progress,sp.completed,sc.score
from teacher t
left join course c on c.teacher_id = t.teacher_id 
left join video v on v.course_id = c.course_id
left join student_video_progress sp on sp.video_id = v.video_id
left join student s on s.student_id = sp.student_id
left join score sc on sc.course_id = c.course_id and sc.student_id = sp.student_id
where t.teacher_id = :id;";

    let video = match conn.exec_map(
        video_query,
        params! {"id" => id },
        |(
            student_id,
            student_name,
            course_id,
            course_name,
            video_title,
            video_duration,
            progress,
            completed,
            score,
        )| {
            VideosInfo {
                student_id,
                student_name,
                course_id,
                course_name,
                video_title,
                video_duration,
                progress,
                completed,
                score,
            }
        },
    ) {
        Ok(video) => video,
        Err(err) => {
            res.render(Json(VideosInfoResponse {
                success: false,
                message: Some(format!("获取视频信息失败:{:?}", err)),
                videos_info: None,
            }));
            return;
        }
    };
    if video.len() == 0 {
        res.render(Json(VideosInfoResponse {
            success: false,
            message: Some("没有视频信息".to_string()),
            videos_info: None,
        }));
    }
    res.render(Json(VideosInfoResponse {
        success: true,
        message: Some("获取视频信息成功".to_string()),
        videos_info: Some(video),
    }));
}

#[handler]
pub async fn get_course_videos(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let id = req.param::<i64>("id").unwrap();

    let video_query = "select v.*
from teacher t
left join course c on t.teacher_id = c.teacher_id
left join video v on v.course_id = c.course_id
where t.teacher_id = :id;";

    let video = match conn.exec_map(
        video_query,
        params! {"id" => id },
        |(video_id, video_title, video_description, video_url, video_duration, course_id)| {
            CourseVideos {
                video_id,
                video_title,
                video_description,
                video_url,
                video_duration,
                course_id,
            }
        },
    ) {
        Ok(video) => video,
        Err(err) => {
            res.render(Json(CourseVideosResponse {
                success: false,
                message: Some(format!("获取视频信息失败:{:?}", err)),
                course_videos: None,
            }));
            return;
        }
    };
    if video.len() == 0 {
        res.render(Json(CourseVideosResponse {
            success: false,
            message: Some("没有视频信息".to_string()),
            course_videos: None,
        }));
    }
    res.render(Json(CourseVideosResponse {
        success: true,
        message: Some("获取视频信息成功".to_string()),
        course_videos: Some(video),
    }));
}

