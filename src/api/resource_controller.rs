#[cfg(test)]
mod tests {
    #[actix_rt::test]
    async fn create_resource() {
        let mut app = test::init_service(
            App::new()
                .wrap(Cors::default()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600))
                .data(config::db::migrate_and_config_db(":memory:"))
                .wrap(actix_web::middleware::Logger::default())
                // .wrap(crate::middleware::authen_middleware::Authentication)
                // .wrap_fn(|req, srv| {
                    // srv.call(req).map(|res| res)
                // })
                .configure(crate::config::app::config_services)
            ).await;

        let resp = test::TestRequest::post()
            .uri("http://localhost/resource")
            .set(header::ContentType::json())
            .set_payload(r#"{"username":"admin","email":"admin@gmail.com","password":"password"}"#.as_bytes())
            .send_request(&mut app)
            .await;

        let resp = test::TestRequest::post()
            .uri("/api/auth/login")
            .set(header::ContentType::json())
            .set_payload(r#"{"username_or_email":"abc@gmail.com","password":"123456"}"#.as_bytes())
            .send_request(&mut app)
            .await;

        assert_eq!(resp.status(), StatusCode::CREATED)
        }
    }
}
