use crate::model::*;
use mysql::prelude::TextQuery;
use salvo::prelude::*;

#[handler]
pub async fn get_courses(depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let course_query = format!("SELECT * from Course");

    let courses = course_query
        .map(
            &mut conn,
            |(course_id, course_name, credit, teacher_id, classroom, schedule, description)| {
                Course {
                    course_id,
                    course_name,
                    credit,
                    teacher_id,
                    classroom,
                    schedule,
                    description,
                }
            },
        )
        .unwrap();

    if courses.len() == 0 {
        res.render(Json(CourseResponse {
            success: false,
            message: Some("获取课程信息失败".to_string()),
            courses: None,
        }));
    }
    res.render(Json(CourseResponse {
        success: true,
        message: Some("课程信息获取成功".to_string()),
        courses: Some(courses),
    }));
    // res.render(format!("get_courses"));
}
