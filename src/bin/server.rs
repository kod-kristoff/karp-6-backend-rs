
use karp::config;
use std::{env, io};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    use actix_web::{App, HttpServer};

    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let app_url = format!("{}:{}", "127.0.0.1", 8080);
    println!("Starting server on http://{}", app_url);

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .configure(config::app::config_services)
    })
    .bind(&app_url)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, HttpServer};

    #[actix_rt::test]
    async fn test_startup_without_auth_middleware_ok() {
        HttpServer::new(move || {
            App::new()
                .wrap(actix_web::middleware::Logger::default())
                .configure(config::app::config_services)
        })
        .bind("localhost:8001".to_string())
        .unwrap()
        .run();

        assert_eq!(true, true);
    }
}
