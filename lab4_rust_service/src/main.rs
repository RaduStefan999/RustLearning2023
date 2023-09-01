mod api;

use api::data::{get_data, get_malina};

use actix_web::{HttpServer, App, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_VACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
        .wrap(logger)
        .service(get_data)
        .service(get_malina)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
