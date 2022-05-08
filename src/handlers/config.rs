use super::prelude::*;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(board::config);
    cfg.configure(category::config);
    cfg.configure(family::config);
    cfg.configure(father_occupation::config);
    cfg.configure(filter::config);
    cfg.configure(gender::config);
    cfg.configure(mother_occupation::config);
    cfg.configure(personal::config);
    cfg.configure(pg::config);
    cfg.configure(qualification::config);
    cfg.configure(specialization::config);
    cfg.configure(student::config);
    cfg.configure(teacher::config);
}
