use actix_web::{get, post, web, Either, HttpResponse, Responder};
use entity::entity::prelude::Specialization;
use entity::sea_orm::ActiveValue::NotSet;
use entity::sea_orm::EntityTrait;
use entity::{entity::specialization, sea_orm::Set};

use crate::AppState;

#[post("/specialization")]
async fn post_specialization(
    info: web::Json<specialization::Model>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let specialization_model = specialization::ActiveModel {
        id: NotSet,
        value: Set(info.value.to_owned()),
    };

    let result = Specialization::insert(specialization_model)
        .exec_with_returning(db_connection)
        .await;

    match result {
        Ok(model) => Either::Left(HttpResponse::Ok().json(model)),
        Err(error) => Either::Right(HttpResponse::InternalServerError().json(error.to_string())),
    }
}

#[get("/specialization")]
async fn get_specializations(state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let specializations = Specialization::find().all(db_connection).await;

    match specializations {
        Ok(specializations) => Either::Left(HttpResponse::Ok().json(specializations)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

#[get("/specialization/{id}")]
async fn get_specialization_by_id(
    specialization_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let specialization = Specialization::find_by_id(specialization_id.to_owned())
        .one(db_connection)
        .await;

    match specialization {
        Ok(specialization) => Either::Left(HttpResponse::Ok().json(specialization)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(post_specialization)
        .service(get_specializations)
        .service(get_specialization_by_id);
}
