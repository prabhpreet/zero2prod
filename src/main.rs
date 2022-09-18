//use env_logger::Env;
use mobexplorezero2prod::email_client::EmailClient;
use mobexplorezero2prod::telemetry::*;
use mobexplorezero2prod::{
    configuration::get_configuration,
    startup::{run, Application},
};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;

//#[actix_web::main] // or
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
