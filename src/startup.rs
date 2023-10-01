use actix_files::Files;
use hyper_util::rt::TokioIo;
use lazy_static::lazy_static;
use tera::Tera;
use tokio::net::TcpListener;

use crate::{handlers, routes::health_check};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let srv = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(TEMPLATES.clone()))
            .wrap(middleware::Logger::default())
            .service(Files::new("/static", "static/").use_last_modified(true))
            .route("/health_check", web::get().to(health_check))
            .service(handlers::index)
            .service(handlers::post)
    })
    .listen(listener)?
    .run();

    let server = warp::serve(filter);

    Ok(srv)
}
