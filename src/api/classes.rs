use crate::model::*;
use salvo::prelude::*;
use sqlx::Row;

#[handler]
pub async fn get_classes(depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.expect("Failed to get database connection");

    let query = "SELECT class_id, class_name FROM class";

    // 执行查询并处理结果
    match sqlx::query(query)
        .fetch_all(&mut *conn)
        .await
    {
        Ok(rows) => {
            // 将查询结果映射到 Class 结构体
            let classes: Vec<Class> = rows.into_iter().map(|row| {
                Class {
                    class_id: row.get("class_id"),
                    class_name: row.get("class_name"),
                }
            }).collect();
            
            // 检查班级列表是否为空
            if classes.is_empty() {
                res.render(Json(ClassesResponse {
                    success: false,
                    message: Some("没有找到班级信息".to_string()),
                    classes: None,
                }));
            } else {
                res.render(Json(ClassesResponse {
                    success: true,
                    message: Some("班级信息获取成功".to_string()),
                    classes: Some(classes),
                }));
            }
        }
        Err(e) => {
            // 处理数据库错误
            res.render(Json(ClassesResponse {
                success: false,
                message: Some(format!("获取班级信息失败: {}", e)),
                classes: None,
            }));
        }
    }
}