use crate::model::*;
use salvo::prelude::*;
use sqlx::Row;

#[handler]
pub async fn get_students_info(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db
        .get_connection()
        .await
        .expect("Failed to get database connection");

    let id = req.param::<i64>("id").unwrap();

    let query = "SELECT st.student_id, st.student_name, c.course_name, cl.class_name
                FROM teacher t
                LEFT JOIN course c ON c.teacher_id = t.teacher_id 
                LEFT JOIN score s ON s.course_id = c.course_id
                LEFT JOIN student st ON st.student_id = s.student_id
                LEFT JOIN class cl ON cl.class_id = st.class_id
                WHERE t.teacher_id = ?";

    match sqlx::query(query).bind(id).fetch_all(&mut *conn).await {
        Ok(rows) => {
            let students_info: Vec<StudentsInfo> = rows
                .into_iter()
                .map(|row| StudentsInfo {
                    student_id: row.get("student_id"),
                    student_name: row.get("student_name"),
                    class_name: row.get("class_name"),
                    course_name: row.get("course_name"),
                })
                .collect();

            if students_info.is_empty() {
                res.render(Json(StudentsInfoResponse {
                    success: false,
                    message: Some("没有找到学生信息".to_string()),
                    students_info: None,
                }));
            } else {
                res.render(Json(StudentsInfoResponse {
                    success: true,
                    message: Some("获取学生信息成功".to_string()),
                    students_info: Some(students_info),
                }));
            }
        }
        Err(err) => {
            res.render(Json(StudentsInfoResponse {
                success: false,
                message: Some(format!("获取学生信息失败: {}", err)),
                students_info: None,
            }));
        }
    }
}

#[handler]
pub async fn add_score(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db
        .get_connection()
        .await
        .expect("Failed to get database connection");

    let score_req = match req.parse_json::<ScoreRequest>().await {
        Ok(data) => data,
        Err(e) => {
            res.render(Json(RegisterResponse {
                success: false,
                message: Some(format!("请求数据解析失败: {}", e)),
            }));
            return;
        }
    };

    let query = "UPDATE score SET score = ? 
                WHERE student_id = ? AND course_id = ?";

    match sqlx::query(query)
        .bind(score_req.score)
        .bind(score_req.student_id)
        .bind(score_req.course_id)
        .execute(&mut *conn)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                res.render(Json(RegisterResponse {
                    success: true,
                    message: Some("成绩更新成功".to_string()),
                }));
            } else {
                res.render(Json(RegisterResponse {
                    success: false,
                    message: Some("未找到匹配的记录".to_string()),
                }));
            }
        }
        Err(e) => {
            res.render(Json(RegisterResponse {
                success: false,
                message: Some(format!("成绩更新失败: {}", e)),
            }));
        }
    }
}
