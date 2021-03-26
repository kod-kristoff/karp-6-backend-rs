use crate::api::*;
use actix_web::web;
use log::info;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");
    cfg.service(
        web::scope("/documentation").service(
            web::resource("spec.yaml")
                .route(web::get().to(documentation_controller::openapi_spec_yaml)),
        ),
    );
}
