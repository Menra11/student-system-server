use crate::model::*;
use mysql::prelude::TextQuery;
use salvo::prelude::*;

#[handler]
pub async fn get_scores(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let id = req.param::<i64>("id").unwrap();

    let score_query =format!("SELECT sc.score_id, c.course_name, c.credit, t.teacher_name, sc.score, sc.semester
      FROM Score sc
      JOIN Course c ON sc.course_id = c.course_id
      JOIN Teacher t ON c.teacher_id = t.teacher_id
      WHERE sc.student_id = {id}
      ORDER BY sc.semester DESC");

    let scores = score_query
        .map(
            &mut conn,
            |(score_id, course_name, credit, teacher_name, score, semester)| Score {
                score_id,
                course_name,
                credit,
                teacher_name,
                score,
                semester,
            },
        )
        .unwrap();
    if scores.len() == 0 {
        res.render(Json(ScoreResponse {
            success: false,
            message: Some("获取成绩信息失败".to_string()),
            scores: None,
        }));
    }

    res.render(Json(ScoreResponse {
        success: true,
        message: Some("成绩获取成功".to_string()),
        scores: Some(scores),
    }));
  // res.render(format!("get_score:{:?}",id));
}
