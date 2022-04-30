use actix_web::{get, post, web, Either, HttpResponse, Responder};
use entity::entity::prelude::Qualification;
use entity::sea_orm::EntityTrait;
use entity::{entity::qualification, sea_orm::Set};

use crate::AppState;

#[post("/qualification")]
async fn post_qualification(
    info: web::Json<qualification::Model>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let qualification_model = qualification::ActiveModel {
        id: Set(info.id.to_owned()),
        value: Set(info.value.to_owned()),
    };

    let result = Qualification::insert(qualification_model)
        .exec_with_returning(db_connection)
        .await;

    match result {
        Ok(model) => Either::Left(HttpResponse::Ok().json(model)),
        Err(error) => Either::Right(HttpResponse::InternalServerError().json(error.to_string())),
    }
}

#[get("/qualification")]
async fn get_qualifications(state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let qualifications = Qualification::find().all(db_connection).await;

    match qualifications {
        Ok(qualifications) => Either::Left(HttpResponse::Ok().json(qualifications)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

#[get("/qualification/{id}")]
async fn get_qualification_by_id(
    qualification_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let qualification = Qualification::find_by_id(qualification_id.to_owned())
        .one(db_connection)
        .await;

    match qualification {
        Ok(qualification) => Either::Left(HttpResponse::Ok().json(qualification)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(post_qualification)
        .service(get_qualifications)
        .service(get_qualification_by_id);
}
