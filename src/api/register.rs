use crate::model::*;
use bcrypt::DEFAULT_COST;
use salvo::prelude::*;
// use sqlx::MySqlConnection;

#[handler]
pub async fn get_register(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db
        .get_connection()
        .await
        .expect("Failed to get database connection");

    // 解析注册数据
    let register_data = match req.parse_json::<RegisterDataRequest<RegisterData>>().await {
        Ok(data) => data,
        Err(e) => {
            res.status_code(salvo::http::StatusCode::BAD_REQUEST);
            res.render(Json(RegisterResponse {
                success: false,
                message: Some(format!("请求数据解析失败: {}", e)),
            }));
            return;
        }
    };

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

    // 检查密码是否存在
    let password = match password {
        Some(p) => p,
        None => {
            res.status_code(salvo::http::StatusCode::NOT_FOUND);
            res.render(Json(RegisterResponse {
                success: false,
                message: Some("密码不能为空".to_string()),
            }));
            return;
        }
    };

    // 生成密码哈希
    let password_hash = match bcrypt::hash(password.clone(), DEFAULT_COST) {
        Ok(hash) => hash,
        Err(e) => {
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(RegisterResponse {
                success: false,
                message: Some(format!("密码加密失败: {}", e)),
            }));
            return;
        }
    };

    let query = "INSERT INTO Student (student_id, student_name, gender, birth_date, class_id, phone, email,password, password_hash) 
             VALUES (?, ?, ?, ?, ?, ?, ?,?, ?)";

    match sqlx::query(query)
        .bind(student_id)
        .bind(student_name)
        .bind(gender)
        .bind(birth_date)
        .bind(class_id)
        .bind(phone)
        .bind(email)
        .bind(password)
        .bind(password_hash)
        .execute(&mut *conn)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                res.render(Json(RegisterResponse {
                    success: true,
                    message: Some("注册成功".to_string()),
                }));
            } else {
                // 插入成功但未影响任何行
                res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
                res.render(Json(RegisterResponse {
                    success: false,
                    message: Some("注册失败，未添加任何记录".to_string()),
                }));
                return;
            }
        }
        Err(e) => {
            // 处理数据库错误
            let message = match &e {
                sqlx::Error::Database(err) if err.is_unique_violation() => {
                    "该学号已被注册".to_string()
                }
                sqlx::Error::Database(err) if err.is_foreign_key_violation() => {
                    "班级ID不存在".to_string()
                }
                _ => format!("数据库错误: {}", e),
            };

            res.status_code(salvo::http::StatusCode::BAD_REQUEST);
            res.render(Json(RegisterResponse {
                success: false,
                message: Some(message),
            }));
        }
    }
}
