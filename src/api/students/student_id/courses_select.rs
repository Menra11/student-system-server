use crate::model::*;
use salvo::prelude::*;
use sqlx::{Acquire, Row};

#[handler]
pub async fn post_courses(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.expect("Failed to get database connection");

    let id = req.param::<i64>("id").unwrap();

    let courses_data = match req.parse_json::<CoursesId>().await {
        Ok(data) => data,
        Err(e) => {
            res.render(Json(CoursesSelectResponse {
                success: false,
                message: Some(format!("选课数据解析失败: {}", e)),
            }));
            return;
        }
    };

    // 使用事务确保所有操作原子性
    let mut tx = match conn.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            res.render(Json(CoursesSelectResponse {
                success: false,
                message: Some(format!("事务启动失败: {}", e)),
            }));
            return;
        }
    };

    for course_id in &courses_data.courses_id {
        // 添加选课记录
        match sqlx::query("INSERT INTO score (student_id, course_id, semester) VALUES (?, ?, '2024-2025-1')")
            .bind(id)
            .bind(course_id)
            .execute(&mut *tx)
            .await
        {
            Ok(_) => {}
            Err(e) => {
                // 如果是主键冲突错误（选课已存在），继续处理其他课程
                 if let Some(db_err) = e.as_database_error() {
                    // MySQL 的重复键错误码是 1062
                    if db_err.code().as_deref() == Some("1062") {
                        // 如果是重复键错误，继续处理其他课程
                        continue;
                    }
                }
            }
        }

        // 为课程的所有视频创建进度记录
        match sqlx::query("SELECT video_id FROM video WHERE course_id = ?")
            .bind(course_id)
            .fetch_all(&mut *tx)
            .await
        {
            Ok(rows) => {
                for row in rows {
                    let video_id: i64 = row.get("video_id");
                    
                    // 修复：使用更安全的 ON DUPLICATE KEY UPDATE 语法
                    match sqlx::query(
                        "INSERT INTO student_video_progress (student_id, video_id, progress, completed) 
                         VALUES (?, ?, 0, 0)
                         ON DUPLICATE KEY UPDATE progress = VALUES(progress), completed = VALUES(completed)"
                    )
                    .bind(id)
                    .bind(video_id)
                    .execute(&mut *tx)
                    .await
                    {
                        Ok(_) => {}
                        Err(e) => {
                            let _ = tx.rollback().await;
                            res.render(Json(CoursesSelectResponse {
                                success: false,
                                message: Some(format!("初始化视频进度失败: {}", e)),
                            }));
                            return;
                        }
                    }
                }
            }
            Err(e) => {
                let _ = tx.rollback().await;
                res.render(Json(CoursesSelectResponse {
                    success: false,
                    message: Some(format!("获取视频信息失败: {}", e)),
                }));
                return;
            }
        }
    }

    // 提交事务
    match tx.commit().await {
        Ok(_) => {
            res.render(Json(CoursesSelectResponse {
                success: true,
                message: Some("选课成功".to_string()),
            }));
        }
        Err(e) => {
            res.render(Json(CoursesSelectResponse {
                success: false,
                message: Some(format!("事务提交失败: {}", e)),
            }));
        }
    }
}