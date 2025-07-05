use crate::model::*;
use salvo::prelude::*;
use sqlx::{Acquire, Row};

#[handler]
pub async fn get_student(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db
        .get_connection()
        .await
        .expect("Failed to get database connection");

    let id = req.param::<i64>("id").unwrap();

    let query = "SELECT s.student_id, s.student_name, s.gender, s.birth_date,s.phone, s.email, s.class_id, c.class_name 
                FROM Student s
                LEFT JOIN Class c ON s.class_id = c.class_id
                WHERE s.student_id = ?";

    match sqlx::query(query).bind(id).fetch_optional(&mut *conn).await {
        Ok(Some(row)) => {
            let student = Student {
                student_id: row.get("student_id"),
                student_name: row.get("student_name"),
                gender: row.get("gender"),
                birth_date: row.get("birth_date"),
                phone: row.get("phone"),
                email: row.get("email"),
                class_id: row.get("class_id"),
                class_name: row.get("class_name"),
            };

            res.render(Json(StudentResponse {
                success: true,
                message: Some("获取学生信息成功".to_string()),
                student: Some(student),
            }));
        }
        Ok(None) => {
            res.render(Json(StudentResponse {
                success: false,
                message: Some("未找到该学生信息".to_string()),
                student: None,
            }));
        }
        Err(err) => {
            res.render(Json(StudentResponse {
                success: false,
                message: Some(format!("获取学生信息失败: {}", err)),
                student: None,
            }));
        }
    }
}

#[handler]
pub async fn put_student(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db
        .get_connection()
        .await
        .expect("Failed to get database connection");

    let id = req.param::<i64>("id").unwrap();

    let Student {
        student_id:_,
        student_name,
        gender,
        birth_date,
        class_id,
        class_name:_,
        phone,
        email
    } = req.parse_json::<Student>().await.unwrap();

    let query = "UPDATE student s
        SET 
            s.student_name = ?,
            s.gender = ?,
            s.birth_date = ?,
            s.class_id = ?,
            s.phone = ?,
            s.email = ?
        WHERE student_id = ?;";

    match sqlx::query(query)
        .bind(student_name)
        .bind(gender)
        .bind(birth_date)
        .bind(class_id)
        .bind(phone)
        .bind(email)
        .bind(id)
        .execute(&mut *conn)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                res.render(Json(StudentResponse {
                    success: true,
                    message: Some("学生进度更新成功".to_string()),
                    student: None
                }));
            } else {
                res.render(Json(StudentResponse {
                    success: false,
                    message: Some("未找到匹配的记录".to_string()),
                    student: None
                }));
            }
        }
        Err(e) => {
            res.render(Json(StudentResponse {
                success: false,
                message: Some(format!("学生进度更新失败: {}", e)),
                student: None
            }));
        }
    }
}

// #[handler]
// pub async fn add_student(req: &mut Request, depot: &mut Depot, res: &mut Response) {
//     let db = depot.obtain::<crate::db::Database>().expect("get db fail");
//     let mut conn = db
//         .get_connection()
//         .await
//         .expect("Failed to get database connection");
//     let student_data = match req.parse_json::<Student>().await {
//         Ok(data) => data,
//         Err(e) => {
//             res.render(Json(StudentResponse {
//                 success: false,
//                 message: Some(format!("请求数据解析失败: {}", e)),
//             }));
//             return;
//         }
//     };
//     let Student {
//         student_id,
//         student_name,
//         gender,
//         birth_date,
//         class_id,
//         class_name:_,
//         phone,
//         email
//     } = student_data;
//     let query = "INSERT INTO Student (student_id, student_name, gender, birth_date, class_id, phone, email, password_hash) 
//              VALUES (?, ?, ?, ?, ?, ?, ?, ?)";
//     match sqlx::query(query)
//         .bind(student_id)
//         .bind(student_name)
//         .bind(gender)
//         .bind(birth_date)
//         .bind(class_id)
//         .bind(phone)
//         .bind(email)
//         .bind(password_hash)
//         .execute(&mut *conn)
//         .await
//     {
//         Ok(result) => {
//             if result.rows_affected() > 0 {
//                 res.render(Json(StudentResponse {
//                     success: true,
//                     message: Some("注册成功".to_string()),
//                 }));
//             } else {
//                 // 插入成功但未影响任何行
//                 res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
//                 res.render(Json(StudentResponse {
//                     success: false,
//                     message: Some("注册失败，未添加任何记录".to_string()),
//                 }));
//             }
//         }
//         Err(e) => {
//             // 处理数据库错误
//             let message = match &e {
//                 sqlx::Error::Database(err) if err.is_unique_violation() => {
//                     "该学号已被注册".to_string()
//                 }
//                 sqlx::Error::Database(err) if err.is_foreign_key_violation() => {
//                     "班级ID不存在".to_string()
//                 }
//                 _ => format!("数据库错误: {}", e),
//             };       
//             res.status_code(salvo::http::StatusCode::BAD_REQUEST);
//             res.render(Json(StudentResponse {
//                 success: false,
//                 message: Some(message),
//             }));
//         }
//     }
// }

#[handler]
pub async fn del_student(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db
        .get_connection()
        .await
        .expect("Failed to get database connection");

    let id = req.param::<i64>("id").unwrap();

    
    let mut tx = match conn.begin().await{
        Ok(tx) => tx,
        Err(e) => {
            res.render(Json(StudentResponse {
                success: false,
                message: Some(format!("事务启动失败: {}", e)),
                student:None
            }));
            return;
        }
    };
    
    match sqlx::query("DELETE FROM student_video_progress WHERE student_id = ?").bind(id).execute(&mut *tx).await {
        Ok(_) => {}
        Err(e) => {
            let _ = tx.rollback().await;
            res.render(Json(StudentResponse {
                success: false,
                message: Some(format!("删除失败: {}", e)),
                student:None
            }));
            return;
        }
    }
    match sqlx::query("DELETE FROM score WHERE student_id = ?").bind(id).execute(&mut *tx).await {
        Ok(_) => {}
        Err(e) => {
            let _ = tx.rollback().await;
            res.render(Json(StudentResponse {
                success: false,
                message: Some(format!("删除失败: {}", e)),
                student:None
            }));
            return;
        }
    }
    match sqlx::query("DELETE FROM student WHERE student_id = ?").bind(id).execute(&mut *tx).await {
        Ok(_) => {}
        Err(e) => {
            let _ = tx.rollback().await;
            res.render(Json(StudentResponse {
                success: false,
                message: Some(format!("删除失败: {}", e)),
                student:None
            }));
            return;
        }
    }

    match tx.commit().await {
        Ok(_) => {
            res.render(Json(StudentResponse {
                success: true,
                message: Some("删除学生成功".to_string()),
                student:None
            }));
        }
        Err(e) => {
            res.render(Json(StudentResponse {
                success: false,
                message: Some(format!("事务提交失败: {}", e)),
                student:None
            }));
        }
    }
}
