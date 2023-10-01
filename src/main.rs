use std::net::SocketAddr;

use blog::handlers;
use tokio;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;

#[tokio::main]
async fn main() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let static_files = warp::path("static").and(warp::fs::dir("static/"));

    let health_check = warp::path!("health_check")
        .map(|| warp::reply::html("OK"))
        .with(warp::trace::named("health_check"));

    let index = warp::path::end()
        .and(warp::get())
        .and_then(handlers::index)
        .with(warp::trace::named("index"));

    let post = warp::path("posts")
        .and(warp::path::param::<String>())
        .and(warp::get())
        .and_then(handlers::post)
        .with(warp::trace(
            |info| tracing::info_span!("post", req.path = ?info.path()),
        ));

    let routes = static_files.or(health_check).or(index).or(post);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    warp::serve(routes).run(addr).await;
}
