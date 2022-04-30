use actix_web::{get, post, web, Either, HttpResponse, Responder};
use entity::entity::prelude::Category;
use entity::sea_orm::ActiveValue::NotSet;
use entity::sea_orm::EntityTrait;
use entity::{entity::category, sea_orm::Set};

use crate::AppState;

#[post("/category")]
async fn post_category(
    info: web::Json<category::Model>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let category_model = category::ActiveModel {
        id: NotSet,
        value: Set(info.value.to_owned()),
    };

    let result = Category::insert(category_model)
        .exec_with_returning(db_connection)
        .await;

    match result {
        Ok(model) => Either::Left(HttpResponse::Ok().json(model)),
        Err(error) => Either::Right(HttpResponse::InternalServerError().json(error.to_string())),
    }
}

#[get("/category")]
async fn get_categorys(state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let categorys = Category::find().all(db_connection).await;

    match categorys {
        Ok(categorys) => Either::Left(HttpResponse::Ok().json(categorys)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

#[get("/category/{id}")]
async fn get_category_by_id(
    category_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let category = Category::find_by_id(category_id.to_owned())
        .one(db_connection)
        .await;

    match category {
        Ok(category) => Either::Left(HttpResponse::Ok().json(category)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(post_category)
        .service(get_categorys)
        .service(get_category_by_id);
}
