use crate::model::*;
use chrono::Local;
use salvo::prelude::*;

const UPLOAD_DIR: &str = "./../student-system-on-rust/assets/videos";

#[handler]
pub async fn upload_video_file(req: &mut Request, res: &mut Response) {
    let title: String = req.query::<String>("title").unwrap();

    // 获取上传的文件
    let form_data = match req.form_data().await {
        Ok(form_data) => form_data,
        Err(err) => {
            res.render(Json(VideoTitleResponse {
                success: false,
                message: Some(format!("上传文件失败:{}", err)),
                file_name: None,
            }));
            return;
        }
    };

    let video_file = match form_data.files.get("video") {
        Some(video_file) => video_file,
        None => {
            res.render(Json(VideoTitleResponse {
                success: false,
                message: Some("获取文件失败".to_string()),
                file_name: None,
            }));
            return;
        }
    };
    // 序号小于.的位置删除
    let mut arr = video_file.name().unwrap().split('.');
    let _ = &arr.next();
    let name = &arr.next().unwrap().to_string();
    let file_name = format!(
        "{}-{}.{}",
        Local::now().timestamp().to_string(),
        title,
        name
    );
    let file_url  = format!(
        "{}-{}",
        Local::now().timestamp().to_string(),
        title
    );
    let file_path = format!("{}/{}", UPLOAD_DIR.to_string(), file_name);
    let file_p = file_path.as_str();

    // 将文件写入磁盘temp_dir
    let data = match std::fs::read(video_file.path()) {
        Ok(data) => data,
        Err(e) => {
            res.render(Json(VideoTitleResponse {
                success: false,
                message: Some(format!("读取文件失败: {}", e,)),
                file_name: None,
            }));
            return;
        }
    };
    match std::fs::write(file_p, data) {
        Ok(_) => {
            res.render(Json(VideoTitleResponse {
                success: true,
                message: Some("写入磁盘成功".to_string()),
                file_name: Some(file_url),
            }));
        }
        Err(err) => res.render(Json(VideoTitleResponse {
            success: false,
            message: Some(format!("写入磁盘失败:{:?}", err)),
            file_name: None,
        })),
    }

    // res.render(format!("upload_video_file:{:?},name:{:?}", file_path,file_name));
}

#[handler]
pub async fn del_video_file(req: &mut Request, res: &mut Response) {
    let url = req.query::<String>("url").unwrap();

    let path = format!("{}/{}.mp4", UPLOAD_DIR, url);
    let p = path.as_str();
    // 如果路径存在就删除该文件
    if std::path::Path::new(p).exists() {
        match std::fs::remove_file(p) {
            Ok(_) => {
                res.render(Json(RegisterResponse {
                    success: true,
                    message: Some("删除文件成功".to_string()),
                }));
            }
            Err(e) => {
                println!("{}", e);
                res.render(Json(RegisterResponse {
                    success: false,
                    message: Some("删除文件失败".to_string()),
                }));
            }
        }
    }
}
