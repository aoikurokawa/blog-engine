use crate::routes::{fetch_all_blogs, health_check, post_blog};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/blog", web::post().to(post_blog))
            .route("/blogs", web::get().to(fetch_all_blogs))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
