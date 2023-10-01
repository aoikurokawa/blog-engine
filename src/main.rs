use std::net::SocketAddr;

use blog::handlers;
use warp::Filter;
use tokio;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let health_check = warp::path!("health_check").map(|| warp::reply::html("OK"));

    let index = warp::path::end().and(warp::get()).and_then(handlers::index);

    let post = warp::path!("post")
        .and(warp::path::param::<String>())
        .and(warp::get())
        .and_then(handlers::post);

    let routes = health_check.or(index).or(post).or(warp::fs::dir("static/"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    warp::serve(routes).run(addr).await;
}
