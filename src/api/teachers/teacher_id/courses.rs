use crate::model::*;
use mysql::{prelude::*, *};
use salvo::prelude::*;

#[handler]
pub async fn get_courses_info(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let id = req.param::<i64>("id").unwrap();

    let course_query =
        "select c.course_id,c.course_name,c.credit,c.classroom,c.schedule,c.description
from teacher t
left join course c on c.teacher_id = t.teacher_id
where t.teacher_id = :id;";

    let course = match conn.exec_map(
        course_query,
        params! {"id" => id },
        |(course_id, course_name, credit, classroom, schedule, description)| CoursesInfo {
            course_id,
            course_name,
            credit,
            classroom,
            schedule,
            description,
        },
    ) {
        Ok(course) => course,
        Err(err) => {
            res.render(Json(CoursesInfoResponse {
                success: false,
                message: Some(format!("获取课程信息失败:{:?}", err)),
                courses_info: None,
            }));
            return;
        }
    };
    if course.len() == 0 {
        res.render(Json(CoursesInfoResponse {
            success: false,
            message: Some("没有课程信息".to_string()),
            courses_info: None,
        }));
    }
    res.render(Json(CoursesInfoResponse {
        success: true,
        message: Some("获取课程信息成功".to_string()),
        courses_info: Some(course),
    }));

    // res.render(format!("get_student:{:?}",id));
}
