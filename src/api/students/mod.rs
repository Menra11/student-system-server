pub mod student_id;
pub use student_id::*;

use crate::model::*;
use salvo::prelude::*;
use sqlx::Row;

#[handler]
pub async fn get_students(depot: &mut Depot, res: &mut Response) {

    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();
    
    let query = "SELECT s.student_id,s.student_name,s.gender,s.birth_date,s.class_id,s.phone,s.email,c.class_name 
        FROM Student s
        LEFT JOIN Class c ON s.class_id = c.class_id";
    match sqlx::query(query)
        .fetch_all(&mut *conn)
        .await
    {
        Ok(rows) => {
            // 将查询结果映射到 Student 结构体
            let students: Vec<Student> = rows.into_iter().map(|row| {
                Student {
                    student_id: row.get("student_id"),
                    student_name: row.get("student_name"),
                    gender: row.get("gender"),
                    birth_date: row.get("birth_date"),
                    phone: row.get("phone"),
                    email: row.get("email"),
                    class_id: row.get("class_id"),
                    class_name: row.get("class_name"),
                }
            }).collect();
            
            // 检查教师列表是否为空
            if students.is_empty() {
                res.render(Json(StudentsResponse {
                    success: false,
                    message: Some("没有找到教师信息".to_string()),
                    students: None,
                }));
            } else {
                res.render(Json(StudentsResponse {
                    success: true,
                    message: Some("教师信息获取成功".to_string()),
                    students: Some(students),
                }));
            }
        }
        Err(e) => {
            // 处理数据库错误
            res.render(Json(StudentsResponse {
                success: false,
                message: Some(format!("获取教师信息失败: {}", e)),
                students: None,
            }));
        }
    }
}