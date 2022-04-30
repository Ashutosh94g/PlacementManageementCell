use actix_web::{get, post, web, Either, HttpResponse, Responder};
use entity::entity::prelude::Family;
use entity::sea_orm::EntityTrait;
use entity::{entity::family, sea_orm::Set};

use crate::AppState;

#[post("/family")]
async fn post_family(info: web::Json<family::Model>, state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let family_model = family::ActiveModel {
        id: entity::sea_orm::ActiveValue::NotSet,
        father_name: Set(info.father_name.to_owned()),
        mother_name: Set(info.mother_name.to_owned()),
        father_occupation_id: Set(info.father_occupation_id.to_owned()),
        mother_occupation_id: Set(info.mother_occupation_id.to_owned()),
        address: Set(info.address.to_owned()),
        city: Set(info.city.to_owned()),
        state: Set(info.state.to_owned()),
        zip: Set(info.zip.to_owned()),
        tenth_year: Set(info.tenth_year.to_owned()),
        tenth_percentage: Set(info.tenth_percentage.to_owned()),
        tenth_board_id: Set(info.tenth_board_id.to_owned()),
        twelfth_year: Set(info.twelfth_year.to_owned()),
        twelfth_percentage: Set(info.twelfth_percentage.to_owned()),
        twelfth_board_id: Set(info.twelfth_board_id.to_owned()),
    };

    let result = Family::insert(family_model)
        .exec_with_returning(db_connection)
        .await;

    match result {
        Ok(model) => Either::Left(HttpResponse::Ok().json(model)),
        Err(error) => Either::Right(HttpResponse::InternalServerError().json(error.to_string())),
    }
}

#[get("/family")]
async fn get_familys(state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let familys = Family::find().all(db_connection).await;

    match familys {
        Ok(familys) => Either::Left(HttpResponse::Ok().json(familys)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

#[get("/family/{id}")]
async fn get_family_by_id(family_id: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let family = Family::find_by_id(family_id.to_owned())
        .one(db_connection)
        .await;

    match family {
        Ok(family) => Either::Left(HttpResponse::Ok().json(family)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(post_family)
        .service(get_familys)
        .service(get_family_by_id);
}
