use crate::model::*;
use mysql::{prelude::*};
use salvo::prelude::*;

#[handler]
pub async fn get_teacher(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let id = req.param::<i64>("id").unwrap();

    let teacher_query = format!(
        "SELECT teacher_id,teacher_name,gender,title,birth_date,phone,email 
  FROM teacher 
  where teacher_id = {id};"
    );

    let teacher = teacher_query
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
    if teacher.len() == 0 {
        res.render(Json(TeacherResponse {
            success: false,
            message: Some("没有教师信息".to_string()),
            teacher: None,
        }));
    }

    res.render(Json(TeacherResponse {
        success: true,
        message: Some("获取教师信息成功".to_string()),
        teacher: teacher.into_iter().next(),
    }));

    // res.render(format!("get_teacher:{:?}", id));
}
