use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    dev::Server,
};
use actix_web::{dev::Service as _, http, web, App, HttpServer};
use futures_util::future::FutureExt;
use log::info;
use sqlx::MySqlPool;
use std::net::TcpListener;

use actix_cors::Cors;

use crate::config::app;
use crate::middleware::auth_middleware::Authentication;

pub fn run(listener: TcpListener, db_pool: MySqlPool) -> Result<Server, std::io::Error> {
    info!("Service is running .......");
    let secret_key = Key::generate();
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("auth_test".to_owned())
                    .cookie_secure(true)
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(Duration::minutes(10)),
                    )
                    .build(),
            )
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_origin("http://localhost:3000")
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .app_data(db_pool.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(Authentication)
            .wrap_fn(|req, srv| srv.call(req).map(|res| res))
            .configure(app::config)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
