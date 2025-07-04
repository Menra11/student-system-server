use crate::model::*;
use salvo::prelude::*;
use sqlx::Row;

#[handler]
pub async fn get_teacher(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db
        .get_connection()
        .await
        .expect("Failed to get database connection");

    let id = req.param::<i64>("id").unwrap();

    let query = "SELECT teacher_id, teacher_name, gender, title, birth_date, phone, email 
                FROM teacher 
                WHERE teacher_id = ?";

    match sqlx::query(query).bind(id).fetch_optional(&mut *conn).await {
        Ok(Some(row)) => {
            let teacher = Teacher {
                teacher_id: row.get("teacher_id"),
                teacher_name: row.get("teacher_name"),
                gender: row.get("gender"),
                title: row.get("title"),
                birth_date: row.get("birth_date"),
                phone: row.get("phone"),
                email: row.get("email"),
            };

            res.render(Json(TeacherResponse {
                success: true,
                message: Some("获取教师信息成功".to_string()),
                teacher: Some(teacher),
            }));
        }
        Ok(None) => {
            res.render(Json(TeacherResponse {
                success: false,
                message: Some("没有找到该教师信息".to_string()),
                teacher: None,
            }));
        }
        Err(err) => {
            res.render(Json(TeacherResponse {
                success: false,
                message: Some(format!("获取教师信息失败: {}", err)),
                teacher: None,
            }));
        }
    }
}
