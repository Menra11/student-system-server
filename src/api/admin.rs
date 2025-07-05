use crate::model::*;
use salvo::prelude::*;
use sqlx::Row;

#[handler]
pub async fn get_admin(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db
        .get_connection()
        .await
        .expect("Failed to get database connection");

    let id = req.param::<i64>("id").unwrap();

    let query = "SELECT admin_id,admin_name FROM admin WHERE admin_id = ?";

    match sqlx::query(query).bind(id).fetch_optional(&mut *conn).await {
        Ok(Some(row)) => {
            let admin = Admin {
                admin_id: row.get("admin_id"),
                admin_name: row.get("admin_name"),
            };

            res.render(Json(AdminResponse {
                success: true,
                message: Some("获取管理员信息成功".to_string()),
                admin: Some(admin),
            }));
        }
        Ok(None) => {
            res.render(Json(AdminResponse {
                success: false,
                message: Some("未找到该管理员信息".to_string()),
                admin: None,
            }));
            return ;
        }
        Err(err) => {
            res.render(Json(AdminResponse {
                success: false,
                message: Some(format!("获取管理员信息失败: {}", err)),
                admin: None,
            }));
        }
    }
}
