use actix_web::{get, post, web, Either, HttpResponse, Responder};
use entity::entity::prelude::Personal;
use entity::sea_orm::ActiveValue::NotSet;
use entity::sea_orm::EntityTrait;
use entity::{entity::personal, sea_orm::Set};

use crate::AppState;

#[post("/personal")]
async fn post_personal(
    info: web::Json<personal::Model>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let personal_model = personal::ActiveModel {
        id: NotSet,
        firstname: Set(info.firstname.to_owned()),
        lastname: Set(info.lastname.to_owned()),
        email: Set(info.email.to_owned()),
        phone: Set(info.phone.to_owned()),
        dob: Set(info.dob.to_owned()),
        gender_id: Set(info.gender_id.to_owned()),
        category_id: Set(info.category_id.to_owned()),
        ug_qualification_id: Set(info.ug_qualification_id.to_owned()),
        ug_specialization_id: Set(info.ug_specialization_id.to_owned()),
        ug_startyear: Set(info.ug_startyear.to_owned()),
        ug_endyear: Set(info.ug_endyear.to_owned()),
        ug_cgpa: Set(info.ug_cgpa.to_owned()),
    };

    let result = Personal::insert(personal_model)
        .exec_with_returning(db_connection)
        .await;

    match result {
        Ok(model) => Either::Left(HttpResponse::Ok().json(model)),
        Err(error) => Either::Right(HttpResponse::InternalServerError().json(error.to_string())),
    }
}

#[get("/personal")]
async fn get_personals(state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let personals = Personal::find().all(db_connection).await;

    match personals {
        Ok(personals) => Either::Left(HttpResponse::Ok().json(personals)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

#[get("/personal/{id}")]
async fn get_personal_by_id(
    personal_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let personal = Personal::find_by_id(personal_id.to_owned())
        .one(db_connection)
        .await;

    match personal {
        Ok(personal) => Either::Left(HttpResponse::Ok().json(personal)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(post_personal)
        .service(get_personals)
        .service(get_personal_by_id);
}
