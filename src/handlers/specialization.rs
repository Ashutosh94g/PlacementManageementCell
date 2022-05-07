use actix_web::{delete, get, post, web, Either, HttpResponse, Responder};
use entity::entity::prelude::Specialization;
use entity::sea_orm::ActiveValue::NotSet;
use entity::sea_orm::{ActiveModelTrait, EntityTrait};
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

#[post("/specialization/{id}")]
async fn update_specialization(
    id: web::Path<i32>,
    info: web::Json<specialization::Model>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let specialization = Specialization::find_by_id(id.into_inner())
        .one(db_connection)
        .await;
    if let Ok(specialization) = specialization {
        if let Some(specialization) = specialization {
            let mut specialization_model: specialization::ActiveModel = specialization.into();
            specialization_model.value = Set(info.value.to_owned());
            let result = specialization_model.update(db_connection).await;
            match result {
                Ok(model) => return HttpResponse::Ok().json(model),
                Err(error) => return HttpResponse::InternalServerError().json(error.to_string()),
            };
        } else {
            return HttpResponse::NotFound().json("specialization not found");
        }
    } else {
        return HttpResponse::NotFound().json("specialization not found");
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

#[delete("/specialization/{id}")]
async fn delete_specialization_by_id(
    specialization_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let specialization = Specialization::delete_by_id(specialization_id.to_owned())
        .exec(db_connection)
        .await;

    match specialization {
        Ok(specialization) => Either::Left(HttpResponse::Ok().json(specialization.rows_affected)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(post_specialization)
        .service(update_specialization)
        .service(get_specializations)
        .service(get_specialization_by_id)
        .service(delete_specialization_by_id);
}
