use actix_web::{delete, post, web, Either, HttpResponse, Responder};
use entity::entity::prelude::Teacher;
use entity::sea_orm::ActiveValue::NotSet;
use entity::sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use entity::{entity::teacher, sea_orm::Set};

use crate::utils::{compare_hash, create_hash, create_jwt};
use crate::AppState;

#[post("/register")]
async fn post_teacher(
    info: web::Json<teacher::Model>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let hashed_password = create_hash(info.password.to_owned());
    let teacher_model = teacher::ActiveModel {
        id: NotSet,
        email: Set(info.email.to_owned()),
        password: Set(hashed_password),
    };

    let result = Teacher::insert(teacher_model)
        .exec_with_returning(db_connection)
        .await;

    let token = create_jwt(info.email.to_owned());

    match result {
        Ok(_) => Either::Left(HttpResponse::Ok().json(token)),
        Err(error) => Either::Right(HttpResponse::InternalServerError().json(error.to_string())),
    }
}

#[post("/login")]
async fn login(info: web::Json<teacher::Model>, state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let teacher = Teacher::find()
        .filter(teacher::Column::Email.eq(info.email.to_owned()))
        .one(db_connection)
        .await;

    let teacher = teacher
        .unwrap()
        .expect("Failed due to error in unwraping model");

    let token = create_jwt(info.email.to_owned());

    if compare_hash(teacher.password.to_owned(), info.password.to_owned()) {
        return HttpResponse::Ok().json(token);
    }

    return HttpResponse::Unauthorized().json("Unauthorized");
}

#[delete("/teacher")]
async fn delete_teacher(email: web::Json<String>, state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let teacher = Teacher::find()
        .filter(teacher::Column::Email.eq(email.to_owned()))
        .one(db_connection)
        .await;

    let teacher = teacher
        .expect("Failed due to error in unwraping model")
        .expect("Failed due to error in unwraping model");

    let result = teacher.delete(db_connection).await;

    match result {
        Ok(teacher) => Either::Left(HttpResponse::Ok().json(teacher.rows_affected)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(post_teacher)
        .service(login)
        .service(delete_teacher);
}
