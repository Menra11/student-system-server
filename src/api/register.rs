use crate::model::*;
use mysql::prelude::*;
use salvo::prelude::*;

#[handler]
pub async fn get_register(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let register_data: RegisterDataRequest<RegisterData> = req
        .parse_json::<RegisterDataRequest<RegisterData>>()
        .await
        .unwrap();
      
    let sid = &register_data.user_from.student_id.unwrap();
    let sname = &register_data.user_from.student_name.unwrap();
    let sgender = &register_data.user_from.gender.unwrap();
    let sdate = &register_data.user_from.birth_date.unwrap();
    let cid = &register_data.user_from.class_id.unwrap();
    let phone = &register_data.user_from.phone.unwrap();
    let email = &register_data.user_from.email.unwrap();
    let password = &register_data.user_from.password.unwrap();

    let query = format!("insert into Student (student_id,student_name,gender,birth_date,class_id,phone,email,password) 
    VALUES({sid},'{sname}','{sgender}','{sdate}',{cid},'{phone}','{email}','{password}');");

    match query.run(&mut conn) {
        Ok(_) => {
            res.render(Json(RegisterResponse {
                success: true,
                message: Some("注册成功".to_string()),
            }));
        }
        Err(e) => {
          println!("{}", e);
            res.render(Json(RegisterResponse {
                success: false,
                message: Some("注册失败".to_string()),
            }));
        }
    }
}
