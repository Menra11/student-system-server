use crate::model::*;
use mysql::{prelude::*, *};
use salvo::prelude::*;

#[handler]
pub async fn get_students_info(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let id = req.param::<i64>("id").unwrap();

    let student_query = "select st.student_id,st.student_name,c.course_name,cl.class_name
from teacher t
left join course c on c.teacher_id = t.teacher_id 
left join score s on s.course_id = c.course_id
left join student st on st.student_id = s.student_id
left join class cl on cl.class_id = st.class_id
where t.teacher_id = :id;";

    let student = match conn.exec_map(
        student_query,
        params! {"id" => id },
        |(student_id, student_name, class_name, course_name)| StudentsInfo {
            student_id,
            student_name,
            class_name,
            course_name,
        },
    ) {
        Ok(student) => student,
        Err(err) => {
            res.render(Json(StudentsInfoResponse {
                success: false,
                message: Some(format!("获取学生信息失败:{:?}", err)),
                students_info: None,
            }));
            return;
        }
    };
    if student.len() == 0 {
        res.render(Json(StudentsInfoResponse {
            success: false,
            message: Some("没有学生信息".to_string()),
            students_info: None,
        }));
    }
    res.render(Json(StudentsInfoResponse {
        success: true,
        message: Some("获取学生信息成功".to_string()),
        students_info: Some(student),
    }));

    // res.render(format!("get_student:{:?}",id));
}

#[handler]
pub async fn add_score(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let ScoreRequest {
        student_id,
        course_id,
        score,
    } = req.parse_json::<ScoreRequest>().await.unwrap();
    let query = "UPDATE score SET score = :score WHERE student_id = :student_id AND course_id = :course_id";
    match conn.exec_drop(query, params!{"student_id" => student_id, "course_id" => course_id, "score" => score}) {
        Ok(_) => {
            res.render(Json(RegisterResponse {
                success: true,
                message: Some("添加成功".to_string()),
            }));
        }
        Err(e) => {
            res.render(Json(RegisterResponse {
                success: true,
                message: Some(format!("添加失败：{}",e)),
            }));
        }
    }
}
