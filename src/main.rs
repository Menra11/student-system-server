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
    let db = match Database::new() {
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

    let router = Router::new()
        .hoop(affix_state::inject(db))
        .get(hello)
        .push(Router::with_path("api").push(Router::with_path("login").post(get_login)));
    println!("{:?}", router);
    let service = Service::new(router).hoop(cors);
    Server::new(acceptor).serve(service).await;
}
