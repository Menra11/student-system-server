use crate::model::*;
use mysql::prelude::TextQuery;
use salvo::prelude::*;

pub mod teacher_id;
pub use teacher_id::*;

#[handler]
pub async fn get_teachers(depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let teacher_query =
        format!("SELECT teacher_id,teacher_name,gender,title,birth_date,phone,email from Teacher");

    let teachers = teacher_query
        .map(
            &mut conn,
            |(teacher_id, teacher_name, gender, title, birth_date, phone, email)| Teacher {
                teacher_id,
                teacher_name,
                gender,
                title,
                birth_date,
                phone,
                email,
            },
        )
        .unwrap();
        if teachers.len() == 0 {
        res.render(Json(TeachersResponse {
            success: false,
            message: Some("获取教师信息失败".to_string()),
            teachers: None,
        }));
    }
    res.render(Json(TeachersResponse {
        success: true,
        message: Some("教师信息获取成功".to_string()),
        teachers: Some(teachers),
    }));
    // res.render(format!("get_teachers"));
}
