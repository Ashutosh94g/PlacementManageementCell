use super::prelude::*;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(gender::config);
    cfg.configure(board::config);
    cfg.configure(category::config);
}
