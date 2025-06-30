use crate::model::*;
use mysql::prelude::TextQuery;
use salvo::prelude::*;

#[handler]
pub async fn post_courses(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db = depot.obtain::<crate::db::Database>().expect("get db fail");
    let mut conn = db.get_connection().await.unwrap();

    let id = req.param::<i64>("id").unwrap();

    let courses_data = req.parse_json::<CoursesId>().await.unwrap();

    let courses_id = &courses_data.courses_id;

    for c in courses_id.iter() {

        match format!("INSERT INTO score (student_id, course_id,semester) VALUES ({id}, {c},'2024-2025-1')").run(&mut conn){
            Ok(_) => {
                res.render(Json(CoursesSelectResponse {
                    success: true,
                    message: Some("选课成功".to_string()),
                }));
            },
            Err(e) => {
                res.render(Json(CoursesSelectResponse {
                    success: false,
                    message: Some(format!("选课失败:{}",e)),
                }));
            }
        };

        let videos = match format!("SELECT video_id FROM video WHERE course_id = {c}")
            .map(&mut conn, |video_id:u32| video_id ){
                Ok(videos) => videos,
                Err(e) => {
                    res.render(Json(CoursesSelectResponse {
                        success: false,
                        message: Some(format!("获取视频信息失败:{}",e)),
                    }));
                    return;
                }
            };
        for v in videos.iter() {
            match format!("INSERT INTO student_video_progress (student_id, video_id,progress,completed) VALUES ({id}, {v},0,0);").run(&mut conn){
                Ok(_) => {},
                Err(e) => {
                    res.render(Json(CoursesSelectResponse {
                        success: false,
                        message: Some(format!("初始化视频进度失败:{}",e)),
                    }));
                    return;
                }
            };
        }
    }
    // res.render(format!("post_courses{:?},courses_id:{:?}", id, courses_id));
}
