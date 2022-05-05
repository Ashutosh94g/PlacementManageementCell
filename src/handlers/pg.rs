use actix_web::{delete, get, post, web, Either, HttpResponse, Responder};
use entity::entity::prelude::Pg;
use entity::sea_orm::ActiveValue::NotSet;
use entity::sea_orm::EntityTrait;
use entity::{entity::pg, sea_orm::Set};

use crate::AppState;

#[post("/pg")]
async fn post_pg(info: web::Json<pg::Model>, state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let pg_model = pg::ActiveModel {
        id: NotSet,
        qualification_id: Set(info.qualification_id.to_owned()),
        specialization_id: Set(info.specialization_id.to_owned()),
        startyear: Set(info.startyear.to_owned()),
        endyear: Set(info.endyear.to_owned()),
        cgpa: Set(info.cgpa.to_owned()),
    };

    let result = Pg::insert(pg_model)
        .exec_with_returning(db_connection)
        .await;

    match result {
        Ok(model) => Either::Left(HttpResponse::Ok().json(model)),
        Err(error) => Either::Right(HttpResponse::InternalServerError().json(error.to_string())),
    }
}

#[get("/pg")]
async fn get_pgs(state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let pgs = Pg::find().all(db_connection).await;

    match pgs {
        Ok(pgs) => Either::Left(HttpResponse::Ok().json(pgs)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

#[get("/pg/{id}")]
async fn get_pg_by_id(pg_id: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let pg = Pg::find_by_id(pg_id.to_owned()).one(db_connection).await;

    match pg {
        Ok(pg) => Either::Left(HttpResponse::Ok().json(pg)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

#[delete("/pg/{id}")]
async fn delete_pg_by_id(pg_id: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let pg = Pg::delete_by_id(pg_id.to_owned()).exec(db_connection).await;

    match pg {
        Ok(pg) => Either::Left(HttpResponse::Ok().json(pg.rows_affected)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(post_pg)
        .service(get_pgs)
        .service(get_pg_by_id)
        .service(delete_pg_by_id);
}
