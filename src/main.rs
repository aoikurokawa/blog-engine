use std::{io, net::{SocketAddr, TcpListener}};

use blog::{handlers, runtime, Async};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;

fn main() -> io::Result<()> {
    let runtime = tokio::runtime::Runtime::new()?;
    let _guard = runtime.enter();

    let mut executor = runtime::init()?;
    executor.block_on(start_heart_beat());

    Ok(())
    // let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());

    // tracing_subscriber::fmt()
    //     .with_env_filter(filter)
    //     .with_span_events(FmtSpan::CLOSE)
    //     .init();

    // let static_files = warp::path("static").and(warp::fs::dir("static/"));

    // let health_check = warp::path!("health_check")
    //     .map(|| warp::reply::html("OK"))
    //     .with(warp::trace::named("health_check"));

    // let index = warp::path::end()
    //     .and(warp::get())
    //     .and_then(handlers::index)
    //     .with(warp::trace::named("index"));

    // let post = warp::path("posts")
    //     .and(warp::path::param::<String>())
    //     .and(warp::get())
    //     .and_then(handlers::post)
    //     .with(warp::trace(
    //         |info| tracing::info_span!("post", req.path = ?info.path()),
    //     ));

    // let routes = static_files.or(health_check).or(index).or(post);

    // let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    // warp::serve(routes).run(addr).await;
}

async fn start_heart_beat() -> io::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = Async::bind(addr)?;

    loop {
        let (mut stream, _) = listener.accept().await?;

        let mut buf = [0; 1024];
        loop {
            let n = match stream.read(&mut buf).await {
                Ok(n) if n == 0 => return Err(io::Error::new(io::ErrorKind::NotFound, "")),
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from stream; err = {:?}", e);
                    return Err(io::Error::new(io::ErrorKind::NotFound, ""));
                }
            };

            println!("{n}");
            if let Err(e) = stream.write_all(&buf[0..n]).await {
                eprintln!("failed to write to stream; err = {:?}", e);
                return Err(io::Error::new(io::ErrorKind::NotFound, ""));
            }
        }
    }
}
