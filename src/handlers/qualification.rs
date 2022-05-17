use actix_web::{delete, get, post, web, Either, HttpResponse, Responder};
use entity::entity::prelude::Qualification;
use entity::sea_orm::ActiveValue::NotSet;
use entity::sea_orm::{ActiveModelTrait, EntityTrait};
use entity::{entity::qualification, sea_orm::Set};

use crate::AppState;

#[post("/qualification")]
async fn post_qualification(
    info: web::Json<qualification::Model>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let qualification_model = qualification::ActiveModel {
        id: NotSet,
        value: Set(info.value.to_owned()),
    };

    let result = Qualification::insert(qualification_model)
        .exec_with_returning(db_connection)
        .await;

    match result {
        Ok(model) => Either::Left(HttpResponse::Created().json(model)),
        Err(error) => Either::Right(HttpResponse::Conflict().json(error.to_string())),
    }
}

#[post("/qualification/{id}")]
async fn update_qualification(
    id: web::Path<i32>,
    info: web::Json<qualification::Model>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let qualification = Qualification::find_by_id(id.into_inner())
        .one(db_connection)
        .await;
    if let Ok(qualification) = qualification {
        if let Some(qualification) = qualification {
            let mut qualification_model: qualification::ActiveModel = qualification.into();
            qualification_model.value = Set(info.value.to_owned());
            let result = qualification_model.update(db_connection).await;
            match result {
                Ok(model) => return HttpResponse::NoContent().json(model),
                Err(error) => return HttpResponse::Conflict().json(error.to_string()),
            };
        } else {
            return HttpResponse::NotFound().json("qualification not found");
        }
    } else {
        return HttpResponse::NotFound().json("qualification not found");
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

#[delete("/qualification/{id}")]
async fn delete_qualification_by_id(
    qualification_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let qualification = Qualification::delete_by_id(qualification_id.to_owned())
        .exec(db_connection)
        .await;

    match qualification {
        Ok(qualification) => Either::Left(HttpResponse::Ok().json(qualification.rows_affected)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(post_qualification)
        .service(update_qualification)
        .service(get_qualifications)
        .service(get_qualification_by_id)
        .service(delete_qualification_by_id);
}
