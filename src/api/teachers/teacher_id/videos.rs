use crate::model::*;
use salvo::prelude::*;
use sqlx::Row;

#[handler]
pub async fn get_scores_info(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.expect("Failed to get database connection");

    let id = req.param::<i64>("id").unwrap();

    let query = 
        "SELECT s.student_id, s.student_name, c.course_id, c.course_name,CAST(sc.score AS FLOAT) AS score_f32
         FROM teacher t
         LEFT JOIN course c ON c.teacher_id = t.teacher_id
         LEFT JOIN score sc ON sc.course_id = c.course_id
         LEFT JOIN student s ON s.student_id = sc.student_id
         WHERE t.teacher_id = ?";

    match sqlx::query(query)
        .bind(id)
        .fetch_all(&mut *conn)
        .await
    {
        Ok(rows) => {
            let scores_info: Vec<ScoreInfo> = rows.into_iter().map(|row| {
                ScoreInfo {
                    student_id: row.get("student_id"),
                    student_name: row.get("student_name"),
                    course_id: row.get("course_id"),
                    course_name: row.get("course_name"),
                    score: row.get("score_f32"),
                }
            }).collect();
            
            if scores_info.is_empty() {
                res.render(Json(ScoresInfoResponse {
                    success: false,
                    message: Some("没有找到相关信息".to_string()),
                    scores_info: None,
                }));
            } else {
                res.render(Json(ScoresInfoResponse {
                    success: true,
                    message: Some("获取信息成功".to_string()),
                    scores_info: Some(scores_info),
                }));
            }
        }
        Err(err) => {
            res.render(Json(ScoresInfoResponse {
                success: false,
                message: Some(format!("获取信息失败: {}", err)),
                scores_info: None,
            }));
        }
    }
}

#[handler]
pub async fn get_course_videos(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.expect("Failed to get database connection");

    let id = req.param::<i64>("id").unwrap();

    let query = 
        "SELECT v.video_id, v.video_title, v.video_description, 
                v.video_url, v.video_duration, v.course_id
         FROM teacher t
         LEFT JOIN course c ON t.teacher_id = c.teacher_id
         LEFT JOIN video v ON v.course_id = c.course_id
         WHERE t.teacher_id = ?";

    match sqlx::query(query)
        .bind(id)
        .fetch_all(&mut *conn)
        .await
    {
        Ok(rows) => {
            let course_videos: Vec<CourseVideos> = rows.into_iter().map(|row| {
                CourseVideos {
                    video_id: row.get("video_id"),
                    video_title: row.get("video_title"),
                    video_description: row.get("video_description"),
                    video_url: row.get("video_url"),
                    video_duration: row.get("video_duration"),
                    course_id: row.get("course_id"),
                }
            }).collect();
            
            if course_videos.is_empty() {
                res.render(Json(CourseVideosResponse {
                    success: false,
                    message: Some("没有找到相关课程视频".to_string()),
                    course_videos: None,
                }));
            } else {
                res.render(Json(CourseVideosResponse {
                    success: true,
                    message: Some("获取课程视频成功".to_string()),
                    course_videos: Some(course_videos),
                }));
            }
        }
        Err(err) => {
            res.render(Json(CourseVideosResponse {
                success: false,
                message: Some(format!("获取课程视频失败: {}", err)),
                course_videos: None,
            }));
        }
    }
}