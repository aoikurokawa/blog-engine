use dotenv::dotenv;
use messages_actix::Blog;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix-web=info");
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let app = Blog::new(8998);
    // let app = MessageApp::new(8080);
    app.run().await
}
