pub mod video_id;

pub use video_id::*;


use crate::model::*;
use salvo::prelude::*;


#[handler]
pub async fn get_videos(req: &mut Request, depot: &mut Depot, res: &mut Response) {

    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();
    
    res.render(format!("get_videos"));
}