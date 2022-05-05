use actix_web::{delete, get, post, web, Either, HttpResponse, Responder};
use entity::entity::prelude::Gender;
use entity::sea_orm::ActiveValue::NotSet;
use entity::sea_orm::EntityTrait;
use entity::{entity::gender, sea_orm::Set};

use crate::AppState;

#[post("/gender")]
async fn post_gender(info: web::Json<gender::Model>, state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let gender_model = gender::ActiveModel {
        id: NotSet,
        value: Set(info.value.to_owned()),
    };

    let result = Gender::insert(gender_model)
        .exec_with_returning(db_connection)
        .await;

    match result {
        Ok(model) => Either::Left(HttpResponse::Ok().json(model)),
        Err(error) => Either::Right(HttpResponse::InternalServerError().json(error.to_string())),
    }
}

#[get("/gender")]
async fn get_genders(state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let genders = Gender::find().all(db_connection).await;

    match genders {
        Ok(genders) => Either::Left(HttpResponse::Ok().json(genders)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

#[get("/gender/{id}")]
async fn get_gender_by_id(gender_id: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let gender = Gender::find_by_id(gender_id.to_owned())
        .one(db_connection)
        .await;

    match gender {
        Ok(gender) => Either::Left(HttpResponse::Ok().json(gender)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

#[delete("/gender/{id}")]
async fn delete_gender_by_id(
    gender_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let gender = Gender::delete_by_id(gender_id.to_owned())
        .exec(db_connection)
        .await;

    match gender {
        Ok(gender) => Either::Left(HttpResponse::Ok().json(gender.rows_affected)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(post_gender)
        .service(get_genders)
        .service(get_gender_by_id)
        .service(delete_gender_by_id);
}
