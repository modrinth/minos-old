use actix_web::web;

pub mod register;

pub fn routes_import(cfg: &mut web::ServiceConfig) {
    // Add services here:
    cfg.service(register::register);
}