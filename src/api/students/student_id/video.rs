use crate::model::*;
use salvo::prelude::*;
use sqlx::Row;

#[handler]
pub async fn get_student_videos(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db
        .get_connection()
        .await
        .expect("Failed to get database connection");

    let id = req.param::<i64>("id").unwrap();

    let query = "SELECT v.video_id, v.video_title, v.video_description, v.video_url, 
                        v.video_duration, t.teacher_name, c.course_name, s.completed 
                FROM video v
                RIGHT JOIN student_video_progress s ON s.video_id = v.video_id 
                LEFT JOIN course c ON v.course_id = c.course_id
                LEFT JOIN teacher t ON c.teacher_id = t.teacher_id
                WHERE s.student_id = ?";

    match sqlx::query(query).bind(id).fetch_all(&mut *conn).await {
        Ok(rows) => {
            let videos: Vec<Video> = rows
                .into_iter()
                .map(|row| Video {
                    video_id: row.get("video_id"),
                    video_title: row.get("video_title"),
                    video_description: row.get("video_description"),
                    video_url: row.get("video_url"),
                    video_duration: row.get("video_duration"),
                    teacher_name: row.get("teacher_name"),
                    course_name: row.get("course_name"),
                    completed: row.get("completed"),
                    course_id: None,
                })
                .collect();

            if videos.is_empty() {
                res.render(Json(VideosResponse {
                    success: false,
                    message: Some("没有找到相关视频信息".to_string()),
                    videos: None,
                    progresses: None,
                }));
            } else {
                res.render(Json(VideosResponse {
                    success: true,
                    message: Some("获取视频信息成功".to_string()),
                    videos: Some(videos),
                    progresses: None,
                }));
            }
        }
        Err(e) => {
            res.render(Json(VideosResponse {
                success: false,
                message: Some(format!("获取视频信息失败: {}", e)),
                videos: None,
                progresses: None,
            }));
        }
    }
}

#[handler]
pub async fn get_video_and_progress(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db
        .get_connection()
        .await
        .expect("Failed to get database connection");

    let id = req.param::<i64>("id").unwrap();
    let video_id = req.param::<i64>("video_id").unwrap();

    // 获取视频信息
    let video_query = "SELECT v.video_id, v.video_title, v.video_description, v.video_url, 
                              v.video_duration, t.teacher_name, c.course_name, c.course_id
                       FROM video v
                       LEFT JOIN course c ON v.course_id = c.course_id
                       LEFT JOIN teacher t ON c.teacher_id = t.teacher_id
                       WHERE v.video_id = ?";

    let video_result = sqlx::query(video_query)
        .bind(video_id)
        .fetch_optional(&mut *conn)
        .await;

    // 获取进度信息
    let progress_query = "SELECT progress_id, student_id, video_id, progress, completed
                          FROM student_video_progress
                          WHERE student_id = ? AND video_id = ?";

    let progress_result = sqlx::query(progress_query)
        .bind(id)
        .bind(video_id)
        .fetch_optional(&mut *conn)
        .await;

    // 处理视频结果
    let video = match video_result {
        Ok(Some(row)) => Some(Video {
            video_id: row.get("video_id"),
            video_title: row.get("video_title"),
            video_description: row.get("video_description"),
            video_url: row.get("video_url"),
            video_duration: row.get("video_duration"),
            teacher_name: row.get("teacher_name"),
            course_name: row.get("course_name"),
            completed: None,
            course_id: row.get("course_id"),
        }),
        Ok(None) => None,
        Err(err) => {
            res.render(Json(VideoResponse {
                success: false,
                message: Some(format!("视频获取失败: {}", err)),
                video: None,
                progress: None,
            }));
            return;
        }
    };

    // 处理进度结果
    let progress = match progress_result {
        Ok(Some(row)) => Some(Progress {
            progress_id: row.get("progress_id"),
            student_id: row.get("student_id"),
            video_id: row.get("video_id"),
            progress: row.get("progress"),
            completed: row.get("completed"),
        }),
        Ok(None) => {
            // 如果没有进度记录，尝试创建新的
            let create_query =
                "INSERT INTO student_video_progress (student_id, video_id, progress, completed)
                                VALUES (?, ?, 0, 0)";

            match sqlx::query(create_query)
                .bind(id)
                .bind(video_id)
                .execute(&mut *conn)
                .await
            {
                Ok(_) => {
                    // 创建成功后重新获取进度
                    if let Ok(Some(row)) = sqlx::query(progress_query)
                        .bind(id)
                        .bind(video_id)
                        .fetch_optional(&mut *conn)
                        .await
                    {
                        Some(Progress {
                            progress_id: row.get("progress_id"),
                            student_id: row.get("student_id"),
                            video_id: row.get("video_id"),
                            progress: row.get("progress"),
                            completed: row.get("completed"),
                        })
                    } else {
                        None
                    }
                }
                Err(e) => {
                    res.render(Json(VideoResponse {
                        success: false,
                        message: Some(format!("创建视频进度失败: {}", e)),
                        video: None,
                        progress: None,
                    }));
                    return;
                }
            }
        }
        Err(err) => {
            res.render(Json(VideoResponse {
                success: false,
                message: Some(format!("视频进度获取失败: {}", err)),
                video: None,
                progress: None,
            }));
            return;
        }
    };

    if video.is_none() {
        res.render(Json(VideoResponse {
            success: false,
            message: Some("视频不存在".to_string()),
            video: None,
            progress: None,
        }));
        return;
    }

    res.render(Json(VideoResponse {
        success: true,
        message: Some("视频获取成功".to_string()),
        video: video,
        progress: progress,
    }));
}

#[handler]
pub async fn put_video_and_progress(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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
