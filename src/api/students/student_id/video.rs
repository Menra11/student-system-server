use crate::model::*;
use mysql::{prelude::{Queryable, TextQuery},*};
use salvo::prelude::*;

#[handler]
pub async  fn get_student_videos(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let id = req.param::<i64>("id").unwrap();

    let videos_query = "SELECT v.video_id,v.video_title,v.video_description,v.video_url,v.video_duration,t.teacher_name,c.course_name,s.completed 
      FROM video v
      right join student_video_progress s on s.video_id = v.video_id 
      LEFT JOIN course c ON v.course_id = c.course_id
      left join teacher t on c.teacher_id = t.teacher_id
      where s.student_id = :id;";
      let videos =  match conn.exec_map(videos_query, params!{"id" => id}, |(video_id,video_title,video_description,video_url,video_duration,teacher_name,course_name,completed)| 
      Video {
        video_id,
        video_title,
        video_description,
        video_url,
        video_duration,
        teacher_name,
        course_name,
        completed,
      }) {
        Ok(videos) => videos,
        Err(e) => {
            res.render(Json(VideosResponse {
                success: false,
                message: Some(format!("获取视频信息失败:{}",e)),
                videos: None,
                progresses:None,
            }));
            return;
        }
    };
      if  videos.len() == 0  {
        res.render(Json(VideosResponse {
            success: false,
            message: Some("没有视频信息失败".to_string()),
            videos: None,
                progresses:None,

        }));
    }
    res.render(Json(VideosResponse {
        success: true,
        message: Some("获取成功".to_string()),
        videos: Some(videos),
        progresses:None,
    }));

}

#[handler]
pub async fn get_video_and_progress(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let id = req.param::<i64>("id").unwrap();
    let video_id = req.param::<i64>("video_id").unwrap();

    let video = match format!(
        "SELECT v.video_id,v.video_title,v.video_description,v.video_url,v.video_duration,t.teacher_name,c.course_name 
      FROM video v
      LEFT JOIN course c ON v.course_id = c.course_id
      left join teacher t on c.teacher_id = t.teacher_id
      where v.video_id = {video_id};")
      .map(&mut conn , |(video_id,video_title,video_description,video_url,video_duration,teacher_name,course_name)| Video {
        video_id,
        video_title,
        video_description,
        video_url,
        video_duration,
        teacher_name,
        course_name,
        completed: None,
      }) {
        Ok(video) => video,
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
    let progress = match format!(
        "SELECT *
      FROM student_video_progress sp
      WHERE sp.student_id = {id} AND sp.video_id = {video_id};"
    )
    .map(
        &mut conn,
        |(progress_id, student_id, video_id, progress, completed)| Progress {
            progress_id,
            student_id,
            video_id,
            progress,
            completed,
        },
    ) {
        Ok(progress) => progress,
        Err(err) => {
            res.render(Json(VideoResponse {
                success: false,
                message: Some(format!("视频进度获取失败,正在创建视频进度列表:{}", err)),
                video: None,
                progress: None,
            }));
            match format!(
                "INSERT INTO student_video_progress (student_id, video_id,progress,completed)
        VALUES ({id}, {video_id},0,0);"
            )
            .run(&mut conn)
            {
                Ok(_) => {}
                Err(e) => res.render(Json(VideoResponse {
                    success: false,
                    message: Some(format!("视频进度创建失败:{}", e)),
                    video: None,
                    progress: None,
                })),
            }
            return;
        }
    };

    res.render(Json(VideoResponse {
        success: true,
        message: Some("视频获取成功".to_string()),
        video: video.into_iter().next(),
        progress: progress.into_iter().next(),
    }));

    // res.render(format!("get_video:{:?} get_video:{:?}", id, video_id));
}

#[handler]
pub async fn put_video_and_progress(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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
