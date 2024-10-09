use std::net::TcpListener;
use isotopes::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // let address = "127.0.0.1:4000";

    let listener = TcpListener::bind("127.0.0.1:4000").expect("failed to bind address");
    // let port = listener.local_addr().unwrap().port();
    // println!("{}", port);

    run(listener).await?.await
}
