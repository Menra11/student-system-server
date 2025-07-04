use crate::model::*;
use salvo::prelude::*;
use sqlx::Row;

#[handler]
pub async fn get_student(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.expect("Failed to get database connection");

    let id = req.param::<i64>("id").unwrap();

    let query = "SELECT s.student_id, s.student_name, s.gender, s.birth_date, 
                        s.phone, s.email, s.class_id, c.class_name 
                FROM Student s
                LEFT JOIN Class c ON s.class_id = c.class_id
                WHERE s.student_id = ?";

    match sqlx::query(query)
        .bind(id)
        .fetch_optional(&mut *conn)
        .await
    {
        Ok(Some(row)) => {
            let student = Student {
                student_id: row.get("student_id"),
                student_name: row.get("student_name"),
                gender: row.get("gender"),
                birth_date: row.get("birth_date"),
                phone: row.get("phone"),
                email: row.get("email"),
                class_id: row.get("class_id"),
                class_name: row.get("class_name"),
            };
            
            res.render(Json(StudentResponse {
                success: true,
                message: Some("获取学生信息成功".to_string()),
                student: Some(student),
            }));
        }
        Ok(None) => {
            res.render(Json(StudentResponse {
                success: false,
                message: Some("未找到该学生信息".to_string()),
                student: None,
            }));
        }
        Err(err) => {
            res.render(Json(StudentResponse {
                success: false,
                message: Some(format!("获取学生信息失败: {}", err)),
                student: None,
            }));
        }
    }
}