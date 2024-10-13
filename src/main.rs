use std::net::TcpListener;
use isotopes::configuration::get_configuration;
use isotopes::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // let address = "127.0.0.1:4000";

    let configuration = get_configuration()
        .expect("Failed to read configuration.");

    let address = format!(
        "127.0.0.1:{}", configuration.application_port
    );

    let listener = TcpListener::bind(address)?;

    // let listener = TcpListener::bind("127.0.0.1:4000").expect("failed to bind address");
    // let port = listener.local_addr().unwrap().port();
    // println!("{}", port);

    run(listener).await?.await
}
