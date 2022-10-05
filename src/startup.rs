use actix_web::middleware::Logger;
use sqlx::PgPool;
use crate::routes::subscribe;
use crate::routes::health_check;
use std::net::TcpListener;
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;

pub fn run(
    listener: TcpListener,
    db_pool: PgPool) -> Result<Server, std::io::Error> {

    // wrap connection in a smart pointer
    let db_pool = web::Data::new(db_pool);
    
    let server = HttpServer::new(move ||  {
        App::new()
            // Middlewares are added using the wrap method on app
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // get a pointer copy and attach it to the application state
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}