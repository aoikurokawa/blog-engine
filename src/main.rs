use std::net::SocketAddr;

use blog::startup::run;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await?;
    run(listener)?.await?;
    Ok(())
}
