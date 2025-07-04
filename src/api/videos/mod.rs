pub mod video_id;

pub use video_id::*;


use crate::model::*;
use salvo::prelude::*;
use sqlx::Row;

#[handler]
pub async fn get_videos(depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.expect("Failed to get database connection");

    let query = "SELECT v.video_id, v.video_title, v.video_description, v.video_url, v.video_duration, c.course_name, c.course_id
                FROM video v
                LEFT JOIN course c ON v.course_id = c.course_id";

    // 执行查询并映射结果
    match sqlx::query(query)
        .fetch_all(&mut *conn)
        .await
    {
        Ok(rows) => {
            // 将查询结果映射到 Video 结构体
            let videos: Vec<Video> = rows.iter().map(|row| {
                Video {
                    video_id: row.get("video_id"),
                    video_title: row.get("video_title"),
                    video_description: row.get("video_description"),
                    video_url: row.get("video_url"),
                    video_duration: row.get("video_duration"),
                    course_name: row.get("course_name"),
                    course_id: row.get("course_id"),
                    teacher_name: None,   // 查询中没有包含教师名字，所以设为None
                    completed: None,       // 查询中没有包含完成状态，所以设为None
                }
            }).collect();
            
            res.render(Json(VideosResponse {
                success: true,
                message: None,
                videos: Some(videos),
                progresses: None,
            }));
        }
        Err(e) => {
            res.render(Json(VideosResponse {
                success: false, // 改为false表示查询失败
                message: Some(format!("获取视频列表失败: {}", e)),
                videos: None,
                progresses: None,
            }));
        }
    }
}