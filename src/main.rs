#![allow(non_ascii_idents)]

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body("Xin chào từ Rust API!")
}

async fn echo_message(msg: web::Json<Message>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(msg.0)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server đang chạy tại http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/echo", web::post().to(echo_message))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
