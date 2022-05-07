use actix_web::{delete, get, post, web, Either, HttpResponse, Responder};
use entity::entity::prelude::Pg;
use entity::sea_orm::ActiveValue::NotSet;
use entity::sea_orm::{ActiveModelTrait, EntityTrait};
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

#[post("/pg/{id}")]
async fn update_pg(
    id: web::Path<i32>,
    info: web::Json<pg::Model>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let pg = Pg::find_by_id(id.into_inner()).one(db_connection).await;
    if let Ok(pg) = pg {
        if let Some(pg) = pg {
            let mut pg_model: pg::ActiveModel = pg.into();
            pg_model.qualification_id = Set(info.qualification_id.to_owned());
            pg_model.specialization_id = Set(info.specialization_id.to_owned());
            pg_model.startyear = Set(info.startyear.to_owned());
            pg_model.endyear = Set(info.endyear.to_owned());
            pg_model.cgpa = Set(info.cgpa.to_owned());
            let result = pg_model.update(db_connection).await;
            match result {
                Ok(model) => return HttpResponse::Ok().json(model),
                Err(error) => return HttpResponse::InternalServerError().json(error.to_string()),
            };
        } else {
            return HttpResponse::NotFound().json("pg not found");
        }
    } else {
        return HttpResponse::NotFound().json("pg not found");
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
        .service(update_pg)
        .service(get_pgs)
        .service(get_pg_by_id)
        .service(delete_pg_by_id);
}
