use crate::routes::subscribe;
use crate::routes::health_check;
use std::net::TcpListener;
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // controls where the application should be listening for requests
    // What is the maximum number of concurrent connections that we should allow
    // How many new connections per unit of time?
    // Do we enable Transport Layer Security (TLS)
    // transport level concerns
    let server = HttpServer::new( ||
        // application level logic lives here
        // routing, middleware, request handlers, 
        App::new()
            // routes take a path, and a route instances object. A Route is a Handler with a set of gaurds
            // web::get() is a short cut for Router::new().gaurd(gaurd::Get())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe)))
            
    .listen(listener)?
    .run();

    Ok(server)
}