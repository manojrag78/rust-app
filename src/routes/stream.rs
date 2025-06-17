// src/routes/stream.rs
use actix_web::{get,post, web, HttpResponse, Responder};
use actix_web::http::header;
use futures_util::stream;
use std::time::Duration;
use tokio::time::sleep;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust!")
}


#[get("/sse-stream")]
async fn stream_sse() -> impl Responder {
    let stream           = stream::unfold(0, |count| async move {
        sleep(Duration::from_secs(1)).await;
        let data = format!("data: {}\n\n", count);
        Some((Ok::<_, actix_web::Error>(web::Bytes::from(data)), count + 1))
    });

    HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "text/event-stream"))
        .insert_header((header::CACHE_CONTROL, "no-cache"))
        .insert_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .streaming(stream)
}

// Add POST endpoint (e.g., to receive a message)
#[post("/send-data")]
async fn post_data(body: String) -> impl Responder {
    println!("Received POST data: {}", body);
    HttpResponse::Ok().body(format!("You posted: {}", body))
}

/// Export the route so it can be configured in main
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(stream_sse);
    cfg.service(post_data);
    cfg.service(index);
}
