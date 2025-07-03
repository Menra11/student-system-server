pub mod video_id;

use mysql::prelude::Queryable;
pub use video_id::*;

use crate::model::*;
use salvo::prelude::*;

#[handler]
pub async fn get_videos(depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let query = "SELECT v.video_id,v.video_title,v.video_description,v.video_url,v.video_duration,c.course_name,c.course_id
  FROM video v
  left join course c on v.course_id = c.course_id";
    match conn.query_map(
        query,
        |(video_id, video_title, video_description, video_url, video_duration, course_name,course_id)| {
            Video {
                video_id,
                video_title,
                video_description,
                video_url,
                video_duration,
                course_name,
                course_id,
                teacher_name: None,
                completed: None,
            }
        },
    ) {
        Ok(videos) => {
            res.render(Json(VideosResponse {
                success: true,
                message: None,
                videos: Some(videos),
                progresses: None,
            }));
        }
        Err(e) => {
            res.render(Json(VideosResponse {
                success: true,
                message: Some(format!("获取视频列表失败:{:?}",e).to_string()),
                videos: None,
                progresses: None,
            }));
        }
    }
}
