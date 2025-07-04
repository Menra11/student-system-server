pub mod course_id;

pub use course_id::*;

use crate::model::*;
use salvo::prelude::*;
use sqlx::Row;

#[handler]
pub async fn get_courses(depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.expect("Failed to get database connection");

    let query = "SELECT course_id, course_name, credit, teacher_id, classroom, schedule, description FROM Course";

    match sqlx::query(query)
        .fetch_all(&mut *conn)
        .await
    {
        Ok(rows) => {
            let courses: Vec<Course> = rows.into_iter().map(|row| {
                Course {
                    course_id: row.get("course_id"),
                    course_name: row.get("course_name"),
                    credit: row.get("credit"),
                    teacher_id: row.get("teacher_id"),
                    classroom: row.get("classroom"),
                    schedule: row.get("schedule"),
                    description: row.get("description"),
                }
            }).collect();
            
            if courses.is_empty() {
                res.render(Json(CoursesResponse {
                    success: false,
                    message: Some("没有找到课程信息".to_string()),
                    courses: None,
                }));
            } else {
                res.render(Json(CoursesResponse {
                    success: true,
                    message: Some("课程信息获取成功".to_string()),
                    courses: Some(courses),
                }));
            }
        }
        Err(err) => {
            res.render(Json(CoursesResponse {
                success: false,
                message: Some(format!("获取课程信息失败: {}", err)),
                courses: None,
            }));
        }
    }
}