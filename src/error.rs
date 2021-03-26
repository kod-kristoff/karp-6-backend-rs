use derive_more::{Display, Error};
use actix_web::http;
use std::io;

#[derive(Debug, Display, Error)]
pub enum KarpError {
    #[display(fmt = "io error")]
    IoError(ErrorSource),
}

#[derive(Debug, Display, Error)]
pub enum ErrorSource {
    #[display(fmt = "{}", _0)]
    StdIoError(io::Error),
}

impl actix_web::error::ResponseError for KarpError {
    fn error_response(&self) -> actix_web::HttpResponse {
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
