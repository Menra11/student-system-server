use crate::model::*;
use salvo::prelude::*;
use sqlx::{Acquire, Row};

#[handler]
pub async fn get_video(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db
        .get_connection()
        .await
        .expect("Failed to get database connection");

    let id = req.param::<i64>("id").unwrap();

    let query =
        "SELECT v.video_id, v.video_title, v.video_description, v.video_url, v.video_duration, 
                t.teacher_name, c.course_name, c.course_id
         FROM video v
         LEFT JOIN course c ON v.course_id = c.course_id
         LEFT JOIN teacher t ON c.teacher_id = t.teacher_id
         WHERE v.video_id = ?";

    match sqlx::query(query).bind(id).fetch_optional(&mut *conn).await {
        Ok(Some(row)) => {
            let video = Video {
                video_id: row.get("video_id"),
                video_title: row.get("video_title"),
                video_description: row.get("video_description"),
                video_url: row.get("video_url"),
                video_duration: row.get("video_duration"),
                teacher_name: row.get("teacher_name"),
                course_name: row.get("course_name"),
                course_id: row.get("course_id"),
                completed: None,
            };

            res.render(Json(VideoResponse {
                success: true,
                message: Some("视频获取成功".to_string()),
                video: Some(video),
                progress: None,
            }));
        }
        Ok(None) => {
            res.render(Json(VideoResponse {
                success: false,
                message: Some("视频不存在".to_string()),
                video: None,
                progress: None,
            }));
        }
        Err(err) => {
            res.render(Json(VideoResponse {
                success: false,
                message: Some(format!("视频获取失败: {}", err)),
                video: None,
                progress: None,
            }));
        }
    }
}

#[handler]
pub async fn put_video(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db
        .get_connection()
        .await
        .expect("Failed to get database connection");

    let id = req.param::<i64>("id").unwrap();
    let video_id = req.param::<i64>("video_id").unwrap();

    let progress_req = match req.parse_json::<ProgressRequest>().await {
        Ok(progress_req) => progress_req,
        Err(err) => {
            res.render(Json(VideoResponse {
                success: false,
                message: Some(format!("视频进度数据获取失败: {}", err)),
                video: None,
                progress: None,
            }));
            return;
        }
    };
    let completed = progress_req.completed;
    let progress = progress_req.progress;

    let query = "UPDATE student_video_progress 
                SET completed = ?, progress = ? 
                WHERE student_id = ? AND video_id = ?";

    match sqlx::query(query)
        .bind(completed)
        .bind(progress)
        .bind(id)
        .bind(video_id)
        .execute(&mut *conn)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                res.render(Json(VideoResponse {
                    success: true,
                    message: Some("视频进度更新成功".to_string()),
                    video: None,
                    progress: None,
                }));
            } else {
                res.render(Json(VideoResponse {
                    success: false,
                    message: Some("未找到匹配的记录".to_string()),
                    video: None,
                    progress: None,
                }));
            }
        }
        Err(e) => {
            res.render(Json(VideoResponse {
                success: false,
                message: Some(format!("视频进度更新失败: {}", e)),
                video: None,
                progress: None,
            }));
        }
    }
}

#[handler]
pub async fn add_video(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db
        .get_connection()
        .await
        .expect("Failed to get database connection");

    let video_data = match req.parse_json::<AllVideo>().await {
        Ok(data) => data,
        Err(e) => {
            res.render(Json(RegisterResponse {
                success: false,
                message: Some(format!("请求数据解析失败: {}", e)),
            }));
            return;
        }
    };

    let AllVideo {
        video_title,
        video_description,
        video_url,
        video_duration,
        course_id,
    } = video_data;

    // 使用事务确保所有操作原子性
    let mut tx = match conn.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            res.render(Json(RegisterResponse {
                success: false,
                message: Some(format!("事务启动失败: {}", e)),
            }));
            return;
        }
    };

    let query = "INSERT INTO video 
                (video_title, video_description, video_url, video_duration, course_id)
                VALUES (?, ?, ?, ?, ?)";

    match sqlx::query(query)
        .bind(video_title)
        .bind(video_description)
        .bind(video_url)
        .bind(video_duration)
        .bind(course_id)
        .execute(&mut *tx)
        .await
    {
        Ok(_) => {}
        Err(e) => {
            let _ = tx.rollback().await;
            let message = match &e {
                sqlx::Error::Database(err) if err.is_foreign_key_violation() => {
                    "课程ID不存在".to_string()
                }
                _ => format!("数据库错误: {}", e),
            };
            res.render(Json(RegisterResponse {
                success: false,
                message: Some(message),
            }));
            return;
        }
    }
    let s_query = "select s.student_id, v.video_id
        from  video v
        left join score s on s.course_id = v.course_id
        where s.course_id = ?";

    let rows = match sqlx::query(s_query)
        .bind(course_id)
        .fetch_all(&mut *tx)
        .await
    {
        Ok(rows) => {
           rows
        }
        Err(err) => {
            let _ = tx.rollback().await;
            res.render(Json(RegisterResponse {
                success: false,
                message: Some(format!("更新学生成绩失败: {}", err)),
            }));
            return ;
        }
    };

    let mut error_occurred = false;
    let mut error_message = String::new();

    for row in rows {
        let student_id = row.get::<i64, _>("student_id");
        let video_id = row.get::<i64, _>("video_id");
        let sc_query = "INSERT INTO student_video_progress (student_id, video_id, progress, completed) 
                        VALUES (?, ?, 0, 0)
                        ON DUPLICATE KEY UPDATE progress = VALUES(progress), completed = VALUES(completed)";
        
        if let Err(err) = sqlx::query(sc_query)
            .bind(student_id)
            .bind(video_id)
            .execute(&mut *tx)
            .await
        {
            error_occurred = true;
            error_message = format!("更新学生成绩失败: {}", err);
            break;
        }
    }

    if error_occurred {
        let _ = tx.rollback().await;
        res.render(Json(RegisterResponse {
            success: false,
            message: Some(error_message),
        }));
        return;
    }


    // 提交事务
    match tx.commit().await {
        Ok(_) => {
            res.render(Json(RegisterResponse {
                success: true,
                message: Some("选课成功".to_string()),
            }));
        }
        Err(e) => {
            res.render(Json(RegisterResponse {
                success: false,
                message: Some(format!("事务提交失败: {}", e)),
            }));
        }
    }
}

#[handler]
pub async fn del_video(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db
        .get_connection()
        .await
        .expect("Failed to get database connection");

    let id = req.param::<i64>("id").unwrap();

    let query = "DELETE FROM video WHERE video_id = ?";

    match sqlx::query(query).bind(id).execute(&mut *conn).await {
        Ok(result) => {
            if result.rows_affected() > 0 {
                res.render(Json(RegisterResponse {
                    success: true,
                    message: Some("删除成功".to_string()),
                }));
            } else {
                res.render(Json(RegisterResponse {
                    success: false,
                    message: Some("未找到匹配的视频".to_string()),
                }));
            }
        }
        Err(e) => {
            res.render(Json(RegisterResponse {
                success: false,
                message: Some(format!("删除失败: {}", e)),
            }));
        }
    }
}
