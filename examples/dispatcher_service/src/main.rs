//! Dispatcher service
//!
//! A simple service that accepts multiple sources of data and dispatches them to different places.

use actix_web::{middleware::Logger, web, App, HttpServer};
use dispatcher_service::csv2json::csv_to_json;

// TODO: log4rs

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        "actix_server=info,actix_web=info,dispatcher_service=info",
    );
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let bind = ("127.0.0.1", 8080);
    log::info!("Starting server on: http://{}:{}", bind.0, bind.1);

    HttpServer::new(|| {
        let logger = Logger::default();

        App::new().wrap(logger).service(
            web::scope("/api")
                .service(web::scope("/csv").route("/to_json", web::post().to(csv_to_json)))
                .service(web::scope("/xl"))
                .service(web::scope("/db")),
        )
    })
    .bind(bind)?
    .run()
    .await
}