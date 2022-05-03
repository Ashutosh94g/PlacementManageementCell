use actix_web::{post, web, Either, HttpResponse, Responder};
use entity::entity::prelude::Teacher;
use entity::sea_orm::ActiveValue::NotSet;
use entity::sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
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

// #[get("/teacher")]
// async fn get_teachers(state: web::Data<AppState>) -> impl Responder {
//     let db_connection = &state.db_connection;
//     let teachers = Teacher::find().all(db_connection).await;

//     match teachers {
//         Ok(teachers) => Either::Left(HttpResponse::Ok().json(teachers)),
//         Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
//     }
// }

// #[get("/teacher/{id}")]
// async fn get_teacher_by_id(
//     teacher_id: web::Path<i32>,
//     state: web::Data<AppState>,
// ) -> impl Responder {
//     let db_connection = &state.db_connection;
//     let teacher = Teacher::find_by_id(teacher_id.to_owned())
//         .one(db_connection)
//         .await;

//     match teacher {
//         Ok(teacher) => Either::Left(HttpResponse::Ok().json(teacher)),
//         Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
//     }
// }

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(post_teacher).service(login);
}
