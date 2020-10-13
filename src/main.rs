use actix_web::{middleware, App, HttpServer};

mod config;
mod handler;
mod util;

#[actix_web::main]
// #[cfg(unix)]
async fn main() -> std::io::Result<()> {
    ::std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .configure(config::app::config_services)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
