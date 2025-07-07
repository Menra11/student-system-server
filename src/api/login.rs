use crate::model::*;
use bcrypt::verify;
use jsonwebtoken::{self, EncodingKey};
use salvo::prelude::*;
use sqlx::Row; 
use time::{Duration, OffsetDateTime};

#[handler]
pub async fn get_login(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.expect("Failed to get database connection");

    let login_data: LoginDataRequest<LoginData> = req
        .parse_json::<LoginDataRequest<LoginData>>()
        .await
        .unwrap();

    let u_id = &login_data.user_from.user_id.unwrap();
    let user = &login_data.user_from.user.unwrap();
    let password = &login_data.user_from.password.unwrap();
    
    // token
    let exp = OffsetDateTime::now_utc() + Duration::days(1);
    let claim = JwtClaims {
        userid: *u_id,
        usertype: user.to_string(),
        exp: exp.unix_timestamp(),
    };
    
    dotenvy::dotenv().ok();
    
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_bytes()),
    );

    // 处理用户登录请求
    match user.as_str() {
        "student" => {
            let query = "SELECT student_id, student_name, password_hash FROM student WHERE student_id = ?";
            
            // 使用 sqlx 查询数据库
            let row = match sqlx::query(query)
                .bind(u_id)
                .fetch_optional(&mut *conn)
                .await
            {
                Ok(Some(row)) => row,
                Ok(None) => {
                    res.render(Json(LoginResponse {
                        success: false,
                        message: Some("用户不存在".to_string()),
                        token: None,
                        error_code: Some("USER_NOT_FOUND".to_string()),
                    }));
                    return;
                }
                Err(e) => {
                    eprintln!("数据库查询错误: {:?}", e);
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(Json(LoginResponse {
                        success: false,
                        message: Some("服务器内部错误".to_string()),
                        token: None,
                        error_code: Some("DATABASE_ERROR".to_string()),
                    }));
                    return;
                }
            };
            
            let password_hash: String = row.get("password_hash");
            
            // 验证密码
            if !verify(password, &password_hash).unwrap_or(false) {
                res.render(Json(LoginResponse {
                    success: false,
                    message: Some("密码错误".to_string()),
                    token: None,
                    error_code: Some("INVALID_CREDENTIALS".to_string()),
                }));
                return;
            }
            
            // 返回登录成功的响应
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
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(Json(LoginResponse {
                        success: false,
                        message: Some(format!("token获取失败:{:?}", err).to_string()),
                        token: None,
                        error_code: Some("TOKEN_GENERATION_ERROR".to_string()),
                    }));
                }
            }
        }
        "teacher" => {
            let query = "SELECT teacher_id, teacher_name, password_hash FROM teacher WHERE teacher_id = ?";
            
            let row = match sqlx::query(query)
                .bind(u_id)
                .fetch_optional(&mut *conn)
                .await
            {
                Ok(Some(row)) => row,
                Ok(None) => {
                    res.render(Json(LoginResponse {
                        success: false,
                        message: Some("用户不存在".to_string()),
                        token: None,
                        error_code: Some("USER_NOT_FOUND".to_string()),
                    }));
                    return;
                }
                Err(e) => {
                    eprintln!("数据库查询错误: {:?}", e);
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(Json(LoginResponse {
                        success: false,
                        message: Some("服务器内部错误".to_string()),
                        token: None,
                        error_code: Some("DATABASE_ERROR".to_string()),
                    }));
                    return;
                }
            };
            
            let password_hash: String = row.get("password_hash");
            
            if !verify(password, &password_hash).unwrap_or(false) {
                res.render(Json(LoginResponse {
                    success: false,
                    message: Some("密码错误".to_string()),
                    token: None,
                    error_code: Some("INVALID_CREDENTIALS".to_string()),
                }));
                return;
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
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(Json(LoginResponse {
                        success: false,
                        message: Some(format!("token获取失败:{:?}", err).to_string()),
                        token: None,
                        error_code: Some("TOKEN_GENERATION_ERROR".to_string()),
                    }));
                }
            }
        }
        "admin" => {
            let query = "SELECT admin_id, admin_name, password_hash FROM admin WHERE admin_id = ?";
            
            let row = match sqlx::query(query)
                .bind(u_id)
                .fetch_optional(&mut *conn)
                .await
            {
                Ok(Some(row)) => row,
                Ok(None) => {
                    res.render(Json(LoginResponse {
                        success: false,
                        message: Some("用户不存在".to_string()),
                        token: None,
                        error_code: Some("USER_NOT_FOUND".to_string()),
                    }));
                    return;
                }
                Err(e) => {
                    eprintln!("数据库查询错误: {:?}", e);
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(Json(LoginResponse {
                        success: false,
                        message: Some("服务器内部错误".to_string()),
                        token: None,
                        error_code: Some("DATABASE_ERROR".to_string()),
                    }));
                    return;
                }
            };
            
            let password_hash: String = row.get("password_hash");
            
            if !verify(password, &password_hash).unwrap_or(false) {
                res.render(Json(LoginResponse {
                    success: false,
                    message: Some("密码错误".to_string()),
                    token: None,
                    error_code: Some("INVALID_CREDENTIALS".to_string()),
                }));
                return;
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
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(Json(LoginResponse {
                        success: false,
                        message: Some(format!("token获取失败:{:?}", err).to_string()),
                        token: None,
                        error_code: Some("TOKEN_GENERATION_ERROR".to_string()),
                    }));
                }
            }
        }
        _ => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(LoginResponse {
                success: false,
                message: Some("无效的用户类型".to_string()),
                token: None,
                error_code: Some("INVALID_USER_TYPE".to_string()),
            }));
        }
    }
}