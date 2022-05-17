use actix_web::{delete, get, post, web, Either, HttpResponse, Responder};
use entity::entity::prelude::Personal;
use entity::sea_orm::ActiveValue::NotSet;
use entity::sea_orm::{ActiveModelTrait, EntityTrait};
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
        Ok(model) => Either::Left(HttpResponse::Created().json(model)),
        Err(error) => Either::Right(HttpResponse::Conflict().json(error.to_string())),
    }
}

#[post("/personal/{id}")]
async fn update_personal(
    id: web::Path<i32>,
    info: web::Json<personal::Model>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let personal = Personal::find_by_id(id.into_inner())
        .one(db_connection)
        .await;
    if let Ok(personal) = personal {
        if let Some(personal) = personal {
            let mut personal_model: personal::ActiveModel = personal.into();
            personal_model.firstname = Set(info.firstname.to_owned());
            personal_model.lastname = Set(info.lastname.to_owned());
            personal_model.email = Set(info.email.to_owned());
            personal_model.phone = Set(info.phone.to_owned());
            personal_model.dob = Set(info.dob.to_owned());
            personal_model.gender_id = Set(info.gender_id.to_owned());
            personal_model.category_id = Set(info.category_id.to_owned());
            personal_model.ug_qualification_id = Set(info.ug_qualification_id.to_owned());
            personal_model.ug_specialization_id = Set(info.ug_specialization_id.to_owned());
            personal_model.ug_startyear = Set(info.ug_startyear.to_owned());
            personal_model.ug_endyear = Set(info.ug_endyear.to_owned());
            personal_model.ug_cgpa = Set(info.ug_cgpa.to_owned());
            let result = personal_model.update(db_connection).await;
            match result {
                Ok(model) => return HttpResponse::NoContent().json(model),
                Err(error) => return HttpResponse::Conflict().json(error.to_string()),
            };
        } else {
            return HttpResponse::NotFound().json("personal not found");
        }
    } else {
        return HttpResponse::NotFound().json("personal not found");
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

#[delete("/personal/{id}")]
async fn delete_personal_by_id(
    personal_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let personal = Personal::delete_by_id(personal_id.to_owned())
        .exec(db_connection)
        .await;

    match personal {
        Ok(personal) => Either::Left(HttpResponse::Ok().json(personal.rows_affected)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(post_personal)
        .service(update_personal)
        .service(get_personals)
        .service(get_personal_by_id)
        .service(delete_personal_by_id);
}
