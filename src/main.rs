use mobexplorezero2prod::{configuration::get_configuration, startup::run};
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;

//#[actix_web::main] // or
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
