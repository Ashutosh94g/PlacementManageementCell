use super::prelude::*;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(board::config);
    cfg.configure(category::config);
    cfg.configure(father_occupation::config);
    cfg.configure(gender::config);
    cfg.configure(mother_occupation::config);
    cfg.configure(qualification::config);
}
