use actix_files::NamedFile;
use crate::error::KarpError;

pub async fn openapi_spec_yaml() -> Result<NamedFile, KarpError> {
    use std::path;

    let path = path::PathBuf::from("templates/karp_api_spec.yaml");
    Ok(NamedFile::open(path)?)
}

#[cfg(test)]
mod tests {
    use crate::{config};
    use actix_web::{App, test, http, http::StatusCode};

    #[actix_rt::test]
    async fn get_yaml() {

        let mut app =
            test::init_service(
                App::new()
                    .wrap(actix_web::middleware::Logger::default())
                    .configure(crate::config::app::config_services)
            ).await;

        let request = test::TestRequest::get()
            .uri("http://localhost/documentation/spec.yaml")
            .to_request();

        let response = test::call_service(&mut app, request).await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("Content-Type").unwrap(),
            "text/x-yaml"
        )
    }
}
