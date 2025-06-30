use crate::model::*;
use mysql::prelude::TextQuery;
use salvo::prelude::*;

#[handler]
pub async fn get_video(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let id = req.param::<i64>("id").unwrap();
    let video_id = req.param::<i64>("video_id").unwrap();

    res.render(format!("get_video:{:?} get_video:{:?}",id,video_id));
}

#[handler]
pub async fn post_video(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let id = req.param::<i64>("id").unwrap();
    let video_id = req.param::<i64>("video_id").unwrap();

    res.render(format!("post_video:{:?} post_video:{:?}",id,video_id));
}
