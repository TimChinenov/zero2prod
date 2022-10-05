use env_logger::Env;
use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use std::net::TcpListener;

// we need main to be async, but by default main cannot be async. 
// async in rust uses futures - (a value that may not be there yet)
// futures have to be polled to get their value. Kind of how operator functions in RxJS need to be subscribed to
// tokio is a "procedural macro". Rust macros take a stream of symbols and output a stream of new symbols which are then passed to the compiler
// Macros are responsible for code generation.
#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
