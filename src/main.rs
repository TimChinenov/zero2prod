use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

// we need main to be async, but by default main cannot be async. 
// async in rust uses futures - (a value that may not be there yet)
// futures have to be polled to get their value. Kind of how operator functions in RxJS need to be subscribed to
// tokio is a "procedural macro". Rust macros take a stream of symbols and output a stream of new symbols which are then passed to the compiler
// Macros are responsible for code generation.
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // controls where the application should be listening for requests
    // What is the maximum number of concurrent connections that we should allow
    // How many new connections per unit of time?
    // Do we enable Transport Layer Security (TLS)
    // transport level concerns
    HttpServer::new( || {
        // application level logic lives here
        // routing, middleware, request handlers, 
        App::new()
            // routes take a path, and a route instances object. A Route is a Handler with a set of gaurds
            // web::get() is a short cut for Router::new().gaurd(gaurd::Get())
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/health-check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
