use log::info;
use news_service_rust::configuration::get_configuration;
use news_service_rust::startup::run;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::env;
use std::net::TcpListener;
use std::time::Duration;
extern crate env_logger;
extern crate log;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    env::set_var("RUST_LOG", "actic_web=debug,actix_server=info,info");
    env_logger::init();
    let config = get_configuration().expect("fail to read configuration");
    // let connection_pool=MySqlPool::connect(&config.database.connection_string())
    //                     .await
    //                     .expect("fail connect to database");
    let connection_pool = MySqlPoolOptions::new().idle_timeout(Duration::from_secs(60))
        .max_connections(5)
        .connect(&config.database.connection_string())
        .await
        .expect("fail connect to database");

    let address = format!("127.0.0.1:{}", config.app_port);
    let listener = TcpListener::bind(address).expect("fail to bind to port");

    run(listener, connection_pool)?.await
}
