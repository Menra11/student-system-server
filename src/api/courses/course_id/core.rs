use crate::model::*;
use salvo::prelude::*;
use sqlx::Row;

#[handler]
pub async fn get_course_and_videos(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db
        .get_connection()
        .await
        .expect("Failed to get database connection");

    let id = req.param::<i64>("id").unwrap();

    // 获取视频信息
    let video_query = "SELECT v.video_id,v.video_title,v.video_description,v.video_duration,v.video_url
      FROM Video v
      where v.course_id = ?;";

    let video_result = sqlx::query(video_query)
        .bind(id)
        .fetch_all(&mut *conn)
        .await;

    // 获取进度信息
    let course_query = "SELECT c.course_id,c.course_name,c.credit,t.teacher_name,c.classroom,c.schedule,c.description 
      FROM Course c
      left join Teacher t on c.teacher_id = t.teacher_id
      where c.course_id = ?;";

    let course_result = sqlx::query(course_query)
        .bind(id)
        .fetch_optional(&mut *conn)
        .await;

    // 处理视频结果
    let videos = match video_result {
        Ok(rows) => {
          let videos:Vec<CourseVideoList> = rows.into_iter().map(|row| {
              CourseVideoList {
                  video_id: row.get("video_id"),
                  video_title: row.get("video_title"),
                  video_description: row.get("video_description"),
                  video_url: row.get("video_url"),
                  video_duration: row.get("video_duration"),
              }
          }).collect();
          Some(videos)
        },
        Err(err) => {
            res.render(Json(CourseAndVideosResponse {
                success: false,
                message: Some(format!("视频获取失败: {}", err)),
                videos: None,
                course: None,
            }));
            return;
        }
    };

    let course = match course_result {
        Ok(Some(row)) => Some(CourseAndVideos {
            course_id: row.get("course_id"),
            course_name: row.get("course_name"),
            credit: row.get("credit"),
            teacher_name: row.get("teacher_name"),
            classroom: row.get("classroom"),
            schedule: row.get("schedule"),
            description: row.get("description"),
        }),
        Ok(None) => {
            None
        }
        Err(err) => {
            res.render(Json(CourseAndVideosResponse {
                success: false,
                message: Some(format!("视频进度获取失败: {}", err)),
                videos: None,
                course: None,
            }));
            return;
        }
    };
    res.render(Json(CourseAndVideosResponse {
        success: true,
        message: Some("视频获取成功".to_string()),
        videos: videos,
        course: course,
    }));

}
