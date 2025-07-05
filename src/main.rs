use api::*;
use db::Database;
use salvo::http::{self, Method};
use salvo::{cors::Cors, prelude::*};
mod api;
mod db;
mod model;

#[handler]
async fn hello() -> &'static str {
    "hello123"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let db = match Database::new().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("❌ 数据库连接失败: {}", e);
            std::process::exit(1);
        }
    };

    let cors = Cors::new()
        .allow_origin(["http://localhost:3000", "http://127.0.0.1:3000"])
        .allow_methods(vec![
            Method::GET,
            Method::PUT,
            Method::POST,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(vec![http::header::CONTENT_TYPE])
        .into_handler();

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;

    let router = Router::new().hoop(affix_state::inject(db.clone())).get(hello).push(
        Router::with_path("api")
            .push(Router::with_path("login").post(get_login))
            .push(Router::with_path("register").post(get_register))
            .push(Router::with_path("courses").get(get_courses))
            .push(Router::with_path("teachers").get(get_teachers))
            .push(Router::with_path("students").get(get_students))
            .push(Router::with_path("videos").get(get_videos))
            .push(
                Router::with_path("video_file")
                    .post(upload_video_file)
                    .delete(del_video_file),
            )
            .push(
                Router::with_path("teacher").push(
                    Router::with_path("{id}")
                        .get(get_teacher)
                        .post(add_score)
                        .push(Router::with_path("courses_info").get(get_courses_info))
                        .push(Router::with_path("students_info").get(get_students_info))
                        .push(Router::with_path("scores_info").get(get_scores_info))
                        .push(Router::with_path("course_videos").get(get_course_videos)),
                ),
            )
            .push(
                Router::with_path("video").post(add_video).push(
                    Router::with_path("{id}")
                        .get(get_video)
                        .put(put_video)
                        .delete(del_video),
                ),
            )
            .push(
                Router::with_path("student").push(
                    Router::with_path("{id}")
                        .get(get_student)
                        .push(Router::with_path("scores").get(get_scores))
                        .push(Router::with_path("courses_select").post(post_courses))
                        .push(Router::with_path("videos").get(get_student_videos))
                        .push(
                            Router::with_path("video").push(
                                Router::with_path("{video_id}")
                                    .get(get_video_and_progress)
                                    .put(put_video_and_progress),
                            ),
                        ),
                ),
            ),
    );
    println!("{:?}", router);
    let service = Service::new(router).hoop(cors);
    Server::new(acceptor).serve(service).await;
}
