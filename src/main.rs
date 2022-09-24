use zero2prod::startup::run;
use std::net::TcpListener;

// we need main to be async, but by default main cannot be async. 
// async in rust uses futures - (a value that may not be there yet)
// futures have to be polled to get their value. Kind of how operator functions in RxJS need to be subscribed to
// tokio is a "procedural macro". Rust macros take a stream of symbols and output a stream of new symbols which are then passed to the compiler
// Macros are responsible for code generation.
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    run(listener)?.await
}
