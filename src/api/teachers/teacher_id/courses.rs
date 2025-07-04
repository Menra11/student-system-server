use crate::model::*;
use salvo::prelude::*;
use sqlx::Row;

#[handler]
pub async fn get_courses_info(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db
        .get_connection()
        .await
        .expect("Failed to get database connection");

    let id = req.param::<i64>("id").unwrap();

    let query =
        "SELECT c.course_id, c.course_name, c.credit, c.classroom, c.schedule, c.description
                FROM teacher t
                LEFT JOIN course c ON c.teacher_id = t.teacher_id
                WHERE t.teacher_id = ?";

    match sqlx::query(query).bind(id).fetch_all(&mut *conn).await {
        Ok(rows) => {
            let courses_info: Vec<CoursesInfo> = rows
                .into_iter()
                .map(|row| CoursesInfo {
                    course_id: row.get("course_id"),
                    course_name: row.get("course_name"),
                    credit: row.get("credit"),
                    classroom: row.get("classroom"),
                    schedule: row.get("schedule"),
                    description: row.get("description"),
                })
                .collect();

            if courses_info.is_empty() {
                res.render(Json(CoursesInfoResponse {
                    success: false,
                    message: Some("没有找到课程信息".to_string()),
                    courses_info: None,
                }));
            } else {
                res.render(Json(CoursesInfoResponse {
                    success: true,
                    message: Some("获取课程信息成功".to_string()),
                    courses_info: Some(courses_info),
                }));
            }
        }
        Err(err) => {
            res.render(Json(CoursesInfoResponse {
                success: false,
                message: Some(format!("获取课程信息失败: {}", err)),
                courses_info: None,
            }));
        }
    }
}
