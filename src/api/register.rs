use crate::model::*;
use bcrypt::DEFAULT_COST;
use mysql::{prelude::*, *};
use salvo::prelude::*;

#[handler]
pub async fn get_register(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let register_data: RegisterDataRequest<RegisterData> = req
        .parse_json::<RegisterDataRequest<RegisterData>>()
        .await
        .unwrap();

    let RegisterData {
        student_id,
        student_name,
        gender,
        birth_date,
        class_id,
        phone,
        email,
        password,
    } = &register_data.user_from;

    let password_hash = bcrypt::hash(password.clone().unwrap(), DEFAULT_COST).unwrap();

    let query = "INSERT INTO Student (student_id, student_name, gender, birth_date, class_id, phone, email, password_hash) 
             VALUES (:sid, :sname, :sgender, :sdate, :cid, :phone, :email, :password_hash)";

    match conn.exec_drop(
        query,
        params! {
            "sid" => student_id,
            "sname" => student_name,
            "sgender" => gender,
            "sdate" => birth_date,
            "cid" => class_id,
            "phone" => phone,
            "email" => email,
            "password_hash" => password_hash,
        },
    ) {
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
