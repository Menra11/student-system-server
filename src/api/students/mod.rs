use crate::model::*;
use salvo::prelude::*;

pub mod student_id;
pub use student_id::*;

#[handler]
pub async fn get_students(req: &mut Request, depot: &mut Depot, res: &mut Response) {

    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();
    
    res.render(format!("get_students"));
}