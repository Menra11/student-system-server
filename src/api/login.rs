use crate::model::*;
use jsonwebtoken::{self, EncodingKey};
use mysql::prelude::*;
use salvo::prelude::*;
use time::{Duration, OffsetDateTime};



#[handler]
pub async fn get_login(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let login_data: LoginDataRequest<LoginData> = req
        .parse_json::<LoginDataRequest<LoginData>>()
        .await
        .unwrap();

    let u_id = &login_data.user_from.user_id.unwrap();
    let user = &login_data.user_from.user.unwrap();
    let password = &login_data.user_from.password.unwrap();
    
    // token
    let exp = OffsetDateTime::now_utc() + Duration::days(14);
    let claim = JwtClaims {
        userid: *u_id,
        usertype: user.to_string(),
        exp: exp.unix_timestamp(),
    };
    dotenvy::dotenv().ok();
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claim,
        &EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string()).as_bytes()),
    );

    let query =
        format!("SELECT {user}_id, {user}_name, password FROM {user} WHERE {user}_id = {u_id}");
    // res.render(format!("{:?}", query));
    match user {
        user if user == "student" => {
            let student_data = query
                .map(&mut conn, |(si, sn, p)| StudentLoginData {
                    student_id: si,
                    student_name: sn,
                    password: p,
                })
                .unwrap();
            if student_data.len() == 0 {
                res.render(Json(LoginResponse {
                    success: false,
                    message: Some("用户不存在".to_string()),
                    token: None,
                    error_code:Some("USER_NOT_FOUND".to_string()),
                }));
                return;
            }
            if student_data[0].password != password.to_string() {
                res.render(Json(LoginResponse {
                    success: false,
                    message: Some("密码错误".to_string()),
                    token: None,
                    error_code: Some("INVALID_CREDENTIALS".to_string()),
                }));
            }
            match token {
                Ok(token) => {
                    res.render(Json(LoginResponse {
                        success: true,
                        message: Some("登录成功".to_string()),
                        token: Some(token),
                        error_code: None,
                    }));
                }
                Err(err) => {
                    res.render(Json(LoginResponse {
                        success: false,
                        message: Some(format!("token获取失败:{:?}", err).to_string()),
                        token: None,
                        error_code: Some("1002".to_string()),
                    }));
                }
            }
        }
        user if user == "teacher" => {
            let teacher_data = query
                .map(&mut conn, |(si, sn, p)| TeacherLoginData {
                    teacher_id: si,
                    teacher_name: sn,
                    password: p,
                })
                .unwrap();
            if teacher_data.len() == 0 {
                res.render(Json(LoginResponse {
                    success: false,
                    message: Some("用户不存在".to_string()),
                    token: None,
                    error_code: Some("USER_NOT_FOUND".to_string()),
                }));
                return;
            }
            if teacher_data[0].password != password.to_string() {
                res.render(Json(LoginResponse {
                    success: false,
                    message: Some("密码错误".to_string()),
                    token: None,
                    error_code: Some("INVALID_CREDENTIALS".to_string()),
                }));
            }
            match token {
                Ok(token) => {
                    res.render(Json(LoginResponse {
                        success: true,
                        message: Some("登录成功".to_string()),
                        token: Some(token),
                        error_code: None,
                    }));
                }
                Err(err) => {
                    res.render(Json(LoginResponse {
                        success: false,
                        message: Some(format!("token获取失败:{:?}", err).to_string()),
                        token: None,
                        error_code: Some("1002".to_string()),
                    }));
                }
            }
        }
        user if user == "admin" => {
            let admin_data = query
                .map(&mut conn, |(si, sn, p)| AdminLoginData {
                    admin_id: si,
                    admin_name: sn,
                    password: p,
                })
                .unwrap();
            if admin_data.len() == 0 {
                res.render(Json(LoginResponse {
                    success: false,
                    message: Some("用户不存在".to_string()),
                    token: None,
                    error_code: Some("USER_NOT_FOUND".to_string()),
                }));
                return;
            }
            if admin_data[0].password != password.to_string() {
                res.render(Json(LoginResponse {
                    success: false,
                    message: Some("密码错误".to_string()),
                    token: None,
                    error_code: Some("INVALID_CREDENTIALS".to_string()),
                }));
            }
            match token {
                Ok(token) => {
                    res.render(Json(LoginResponse {
                        success: true,
                        message: Some("登录成功".to_string()),
                        token: Some(token),
                        error_code: None,
                    }));
                }
                Err(err) => {
                    res.render(Json(LoginResponse {
                        success: false,
                        message: Some(format!("token获取失败:{:?}", err).to_string()),
                        token: None,
                        error_code: Some("1002".to_string()),
                    }));
                }
            }
        }
        _ => {
            res.render(format!("error"));
            res.render(Json(LoginResponse {
                success: false,
                message: Some("用户获取失败".to_string()),
                token: None,
                error_code: Some("1002".to_string()),
            }));
        }
    }
}
