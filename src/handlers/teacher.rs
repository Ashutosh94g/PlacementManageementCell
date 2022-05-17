use actix_web::{delete, post, web, Either, HttpResponse, Responder};
use entity::entity::prelude::Teacher;
use entity::sea_orm::ActiveValue::NotSet;
use entity::sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use entity::{entity::teacher, sea_orm::Set};
use serde::Deserialize;

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
        Ok(_) => Either::Left(HttpResponse::Created().json(token)),
        Err(error) => Either::Right(HttpResponse::Conflict().json(error.to_string())),
    }
}

#[derive(Debug, Deserialize)]
struct ChangePassword {
    email: String,
    old_password: String,
    new_password: String,
}

#[post("/update-password")]
async fn change_password(
    info: web::Json<ChangePassword>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let teacher = Teacher::find()
        .filter(teacher::Column::Email.eq(info.email.to_owned()))
        .one(db_connection)
        .await;

    if let Ok(teacher) = teacher {
        if let Some(teacher) = teacher {
            if compare_hash(teacher.password.to_owned(), info.old_password.to_owned()) {
                let new_password = create_hash(info.new_password.to_owned());
                let mut teacher_model: teacher::ActiveModel = teacher.into();
                teacher_model.password = Set(new_password);
                let result = teacher_model.update(db_connection).await;
                match result {
                    Ok(model) => return HttpResponse::NoContent().json(model.email.to_owned()),
                    Err(error) => {
                        return HttpResponse::Conflict().json(error.to_string())
                    }
                };
            } else {
                return HttpResponse::Unauthorized().json("Wrong password");
            }
        } else {
            return HttpResponse::NotFound().json("teacher not found");
        }
    } else {
        return HttpResponse::NotFound().json("teacher not found");
    }
}

#[derive(Debug, Deserialize)]
struct ChangeEmail {
    old_email: String,
    new_email: String,
    password: String,
}

#[post("/update-email")]
async fn change_email(info: web::Json<ChangeEmail>, state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let teacher = Teacher::find()
        .filter(teacher::Column::Email.eq(info.old_email.to_owned()))
        .one(db_connection)
        .await;

    if let Ok(teacher) = teacher {
        if let Some(teacher) = teacher {
            if compare_hash(teacher.password.to_owned(), info.password.to_owned()) {
                let mut teacher_model: teacher::ActiveModel = teacher.into();
                teacher_model.email = Set(info.new_email.to_owned());
                let result = teacher_model.update(db_connection).await;
                match result {
                    Ok(model) => return HttpResponse::Ok().json(model.email.to_owned()),
                    Err(error) => {
                        return HttpResponse::Conflict().json(error.to_string())
                    }
                };
            } else {
                return HttpResponse::Unauthorized().json("Wrong password");
            }
        } else {
            return HttpResponse::NotFound().json("teacher not found");
        }
    } else {
        return HttpResponse::NotFound().json("teacher not found");
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
        .service(change_password)
        .service(change_email)
        .service(login)
        .service(delete_teacher);
}
