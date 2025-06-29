use crate::model::*;
use mysql::prelude::TextQuery;
use salvo::prelude::*;

#[handler]
pub async fn get_student(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let id = req.param::<i64>("id").unwrap();

    let student_query =format!("SELECT s.student_id,s.student_name,s.gender,s.birth_date,s.phone,s.email,s.class_id, c.class_name 
      FROM Student s
      LEFT JOIN Class c ON s.class_id = c.class_id
      WHERE s.student_id = {id}");

    let student = student_query
        .map(
            &mut conn,
            |(id, name, gender, date, phone, email, class_id, class_name)| Student {
                student_id: id,
                student_name: name,
                gender: gender,
                birth_date: date,
                phone: phone,
                email: email,
                class_id: class_id,
                class_name: class_name,
            },
        )
        .unwrap();
    if  student.len() == 0  {
        res.render(Json(StudentResponse {
            success: false,
            message: Some("获取学生信息失败".to_string()),
            student: None,
        }));
    }
    res.render(Json(StudentResponse {
        success: true,
        message: Some("获取成功".to_string()),
        student: student.into_iter().next(),
    }));

    // res.render(format!("get_student:{:?}",id));
}
