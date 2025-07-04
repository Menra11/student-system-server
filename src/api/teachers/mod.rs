pub mod teacher_id;
pub use teacher_id::*;

use crate::model::*;
use salvo::prelude::*;
use sqlx::Row;

#[handler]
pub async fn get_teachers(depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.expect("Failed to get database connection");

    let query = "SELECT teacher_id, teacher_name, gender, title, birth_date, phone, email FROM teacher";

    // 执行查询并处理结果
    match sqlx::query(query)
        .fetch_all(&mut *conn)
        .await
    {
        Ok(rows) => {
            // 将查询结果映射到 Teacher 结构体
            let teachers: Vec<Teacher> = rows.into_iter().map(|row| {
                Teacher {
                    teacher_id: row.get("teacher_id"),
                    teacher_name: row.get("teacher_name"),
                    gender: row.get("gender"),
                    title: row.get("title"),
                    birth_date: row.get("birth_date"),
                    phone: row.get("phone"),
                    email: row.get("email"),
                }
            }).collect();
            
            // 检查教师列表是否为空
            if teachers.is_empty() {
                res.render(Json(TeachersResponse {
                    success: false,
                    message: Some("没有找到教师信息".to_string()),
                    teachers: None,
                }));
            } else {
                res.render(Json(TeachersResponse {
                    success: true,
                    message: Some("教师信息获取成功".to_string()),
                    teachers: Some(teachers),
                }));
            }
        }
        Err(e) => {
            // 处理数据库错误
            res.render(Json(TeachersResponse {
                success: false,
                message: Some(format!("获取教师信息失败: {}", e)),
                teachers: None,
            }));
        }
    }
}