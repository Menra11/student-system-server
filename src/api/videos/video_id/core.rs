use crate::model::*;
use mysql::{
    params,
    prelude::{Queryable, TextQuery},
};
use salvo::prelude::*;

#[handler]
pub async fn get_video(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let id = req.param::<i64>("id").unwrap();

    let query = 
        "SELECT v.video_id,v.video_title,v.video_description,v.video_url,v.video_duration,t.teacher_name,c.course_name 
      FROM video v
      LEFT JOIN course c ON v.course_id = c.course_id
      left join teacher t on c.teacher_id = t.teacher_id
      where v.video_id = :id;";
    match conn.exec_map(query, params!{"id" => id}, |(video_id,video_title,video_description,video_url,video_duration,teacher_name,course_name)| Video {
        video_id,
        video_title,
        video_description,
        video_url,
        video_duration,
        teacher_name,
        course_name,
        completed: None,
        course_id: None,
      })
        {
        Ok(video) => res.render(Json(VideoResponse {
        success: true,
        message: Some("视频获取成功".to_string()),
        video: video.into_iter().next(),
        progress: None,
    })),
        Err(err) => {
                res.render(Json(VideoResponse {
                success: false,
                message: Some(format!("视频获取失败:{}", err)),
                video: None,
                progress: None,
            }));
            return;
        }
    };
}

#[handler]
pub async fn put_video(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let id = req.param::<i64>("id").unwrap();
    let video_id = req.param::<i64>("video_id").unwrap();

    let progress_req = match req.parse_json::<ProgressRequest>().await {
        Ok(progress_req) => progress_req,
        Err(err) => {
            res.render(Json(VideoResponse {
                success: false,
                message: Some(format!("视频进度数据获取失败:{}", err)),
                video: None,
                progress: None,
            }));
            return;
        }
    };
    let completed = progress_req.completed;
    let progress = progress_req.progress;

    match format!(
        "UPDATE student_video_progress SET completed = {completed},progress = {progress} WHERE student_id = {id} AND video_id = {video_id};"
    )
    .run(&mut conn)
    {
        Ok(_) => {res.render(Json(VideoResponse {
                success: true,
                message: Some(format!("视频进度更新成功")),
                video: None,
                progress: None,
            }))}
        Err(e) => {
            res.render(Json(VideoResponse {
                success: false,
                message: Some(format!("视频进度更新失败:{}", e)),
                video: None,
                progress: None,
            }))
        }
    };

    // println!("progress_req:{:?}",progress_req);
    // res.render(format!("post_video:{:?} post_video:{:?},c{:?},p{:?}", id, video_id,completed,progress));
}

#[handler]
pub async fn add_video(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let AllVideo {
        video_title,
        video_description,
        video_url,
        video_duration,
        course_id,
    } = req.parse_json::<AllVideo>().await.unwrap();

    let video_query = "INSERT INTO
  video (video_title, video_description, video_url, video_duration, course_id)
  VALUES (:video_title, :video_description, :video_url, :video_duration, :course_id);";

    match conn.exec_drop(
        video_query,
        params! {
            "video_title" => video_title,
            "video_description" => video_description,
            "video_url" => video_url,
            "video_duration" => video_duration,
            "course_id" => course_id,
        },
    ) {
        Ok(_) => {
            res.render(Json(RegisterResponse {
                success: true,
                message: Some("添加成功".to_string()),
            }));
        }
        Err(e) => {
            res.render(Json(RegisterResponse {
                success: true,
                message: Some(format!("添加失败：{}", e)),
            }));
        }
    }
}

#[handler]
pub async fn del_video(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let id = req.param::<i64>("id").unwrap();

    match conn.exec_drop(
        "DELETE FROM Video WHERE video_id = :id",
        params! {"id" => id},
    ) {
        Ok(_) => {
            res.render(Json(RegisterResponse {
                success: true,
                message: Some("删除成功".to_string()),
            }));
        }
        Err(e) => res.render(Json(RegisterResponse {
            success: false,
            message: Some(format!("删除失败：{}", e)),
        })),
    }
}
