use crate::model::*;
use salvo::prelude::*;
use sqlx::Row;

#[handler]
pub async fn get_scores(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.expect("Failed to get database connection");

    let id = req.param::<i64>("id").unwrap();

    let query = "SELECT sc.score_id, c.course_name, c.credit, t.teacher_name, CAST(sc.score AS FLOAT) AS score_f32 , sc.semester
                FROM Score sc
                JOIN Course c ON sc.course_id = c.course_id
                JOIN Teacher t ON c.teacher_id = t.teacher_id
                WHERE sc.student_id = ?
                ORDER BY sc.semester DESC";

    match sqlx::query(query)
        .bind(id)
        .fetch_all(&mut *conn)
        .await
    {
        Ok(rows) => {
            let scores: Vec<Score> = rows.into_iter().map(|row| {
                Score {
                    score_id: row.get("score_id"),
                    course_name: row.get("course_name"),
                    credit: row.get("credit"),
                    teacher_name: row.get("teacher_name"),
                    score: row.get("score_f32"),
                    semester: row.get("semester"),
                }
            }).collect();
            
            if scores.is_empty() {
                res.render(Json(ScoreResponse {
                    success: false,
                    message: Some("没有找到成绩信息".to_string()),
                    scores: None,
                }));
            } else {
                res.render(Json(ScoreResponse {
                    success: true,
                    message: Some("成绩获取成功".to_string()),
                    scores: Some(scores),
                }));
            }
        }
        Err(err) => {
            res.render(Json(ScoreResponse {
                success: false,
                message: Some(format!("获取成绩信息失败: {}", err)),
                scores: None,
            }));
        }
    }
}