use actix_web::{delete, get, post, web, Either, HttpResponse, Responder};
use entity::entity::prelude::MotherOccupation;
use entity::sea_orm::ActiveValue::NotSet;
use entity::sea_orm::{ActiveModelTrait, EntityTrait};
use entity::{entity::mother_occupation, sea_orm::Set};

use crate::AppState;

#[post("/mother_occupation")]
async fn post_mother_occupation(
    info: web::Json<mother_occupation::Model>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let mother_occupation_model = mother_occupation::ActiveModel {
        id: NotSet,
        value: Set(info.value.to_owned()),
    };

    let result = MotherOccupation::insert(mother_occupation_model)
        .exec_with_returning(db_connection)
        .await;

    match result {
        Ok(model) => Either::Left(HttpResponse::Created().json(model)),
        Err(error) => Either::Right(HttpResponse::Conflict().json(error.to_string())),
    }
}

#[post("/mother_occupation/{id}")]
async fn update_mother_occupation(
    id: web::Path<i32>,
    info: web::Json<mother_occupation::Model>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let mother_occupation = MotherOccupation::find_by_id(id.into_inner())
        .one(db_connection)
        .await;
    if let Ok(mother_occupation) = mother_occupation {
        if let Some(mother_occupation) = mother_occupation {
            let mut mother_occupation_model: mother_occupation::ActiveModel =
                mother_occupation.into();
            mother_occupation_model.value = Set(info.value.to_owned());
            let result = mother_occupation_model.update(db_connection).await;
            match result {
                Ok(model) => return HttpResponse::NoContent().json(model),
                Err(error) => return HttpResponse::Conflict().json(error.to_string()),
            };
        } else {
            return HttpResponse::NotFound().json("mother_occupation not found");
        }
    } else {
        return HttpResponse::NotFound().json("mother_occupation not found");
    }
}

#[get("/mother_occupation")]
async fn get_mother_occupations(state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let mother_occupations = MotherOccupation::find().all(db_connection).await;

    match mother_occupations {
        Ok(mother_occupations) => Either::Left(HttpResponse::Ok().json(mother_occupations)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

#[get("/mother_occupation/{id}")]
async fn get_mother_occupation_by_id(
    mother_occupation_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let mother_occupation = MotherOccupation::find_by_id(mother_occupation_id.to_owned())
        .one(db_connection)
        .await;

    match mother_occupation {
        Ok(mother_occupation) => Either::Left(HttpResponse::Ok().json(mother_occupation)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

#[delete("/mother_occupation/{id}")]
async fn delete_mother_occupation_by_id(
    mother_occupation_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let mother_occupation = MotherOccupation::delete_by_id(mother_occupation_id.to_owned())
        .exec(db_connection)
        .await;

    match mother_occupation {
        Ok(mother_occupation) => {
            Either::Left(HttpResponse::Ok().json(mother_occupation.rows_affected))
        }
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(post_mother_occupation)
        .service(update_mother_occupation)
        .service(get_mother_occupations)
        .service(get_mother_occupation_by_id)
        .service(delete_mother_occupation_by_id);
}
