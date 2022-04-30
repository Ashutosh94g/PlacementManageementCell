use actix_web::{get, post, web, Either, HttpResponse, Responder};
use entity::entity::prelude::FatherOccupation;
use entity::sea_orm::EntityTrait;
use entity::{entity::father_occupation, sea_orm::Set};

use crate::AppState;

#[post("/father_occupation")]
async fn post_father_occupation(
    info: web::Json<father_occupation::Model>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let father_occupation_model = father_occupation::ActiveModel {
        id: Set(info.id.to_owned()),
        value: Set(info.value.to_owned()),
    };

    let result = FatherOccupation::insert(father_occupation_model)
        .exec_with_returning(db_connection)
        .await;

    match result {
        Ok(model) => Either::Left(HttpResponse::Ok().json(model)),
        Err(error) => Either::Right(HttpResponse::InternalServerError().json(error.to_string())),
    }
}

#[get("/father_occupation")]
async fn get_father_occupations(state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let father_occupations = FatherOccupation::find().all(db_connection).await;

    match father_occupations {
        Ok(father_occupations) => Either::Left(HttpResponse::Ok().json(father_occupations)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

#[get("/father_occupation/{id}")]
async fn get_father_occupation_by_id(
    father_occupation_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let father_occupation = FatherOccupation::find_by_id(father_occupation_id.to_owned())
        .one(db_connection)
        .await;

    match father_occupation {
        Ok(father_occupation) => Either::Left(HttpResponse::Ok().json(father_occupation)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(post_father_occupation)
        .service(get_father_occupations)
        .service(get_father_occupation_by_id);
}
