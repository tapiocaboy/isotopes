use std::net::TcpListener;
use isotopes::configuration::get_configuration;
use sqlx::PgPool;
use isotopes::startup::run;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // let address = "127.0.0.1:4000";
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration()
        .expect("Failed to read configuration.");

    let connection = PgPool::connect(
        &configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!(
        "127.0.0.1:{}", configuration.application_port
    );

    let listener = TcpListener::bind(address)?;

    // let listener = TcpListener::bind("127.0.0.1:4000").expect("failed to bind address");
    // let port = listener.local_addr().unwrap().port();
    // println!("{}", port);

    run(listener, connection)?.await
}
