use actix_web::{
    error,
    http,
    web,
    HttpRequest,
    HttpResponse,};
use actix_files::NamedFile;
use derive_more::{Display, Error};
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    use actix_web::{App, HttpServer};

    println!("Starting server on http://127.0.0.1:8080");
    let app_state = web::Data::new(AppState {
        data: String::from("data")
    });
    HttpServer::new(move || {
        App::new()
            .configure(config_app(app_state.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

struct AppState {
    data: String,
}

fn config_app(app_state: web::Data<AppState>) -> Box<dyn Fn(&mut web::ServiceConfig)> {
    Box::new(move |cfg: &mut web::ServiceConfig| {
        cfg.app_data(app_state.clone())
            .service(
                web::resource("/documentation/spec.yaml")
                    .route(web::get().to(get_doc))
            );
    })
}

// Handlers
async fn get_doc() -> Result<NamedFile, KarpError> {
    use std::path;

    let path = path::PathBuf::from("templates/karp_api_spec.yaml");
    Ok(NamedFile::open(path)?)
}

// Error
#[derive(Debug, Display, Error)]
enum KarpError {
    #[display(fmt = "io error")]
    IoError(ErrorSource),
}

#[derive(Debug, Display, Error)]
enum ErrorSource {
    #[display(fmt = "{}", _0)]
    StdIoError(io::Error),
}

impl error::ResponseError for KarpError {
    fn error_response(&self) -> HttpResponse {
        use actix_web::dev::HttpResponseBuilder;
        HttpResponseBuilder::new(self.status_code())
            .set_header(http::header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> http::StatusCode {
        match *self {
            _ => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<io::Error> for KarpError {
    fn from(err: io::Error) -> KarpError {
        KarpError::IoError(ErrorSource::StdIoError(err))
    }
}
mod tests {
    use super::*;
    use actix_web::{test, App};

    mod api {
        use super::*;

        #[actix_rt::test]
        async fn get_yaml() {
            let app_state = web::Data::new(AppState {
                data: String::from("data")
            });
            let mut app = test::init_service(
                App::new()
                    .configure(
                        config_app(app_state.clone())
            )).await;

            let request = test::TestRequest::get()
                .uri("http://localhost/documentation/spec.yaml")
                .to_request();

            let response = test::call_service(&mut app, request).await;

            assert_eq!(response.status(), http::StatusCode::OK);
            assert_eq!(response.headers().get("Content-Type").unwrap(), "text/x-yaml")
        }
    }
}
