use actix_web::{delete, get, post, web, Either, HttpResponse, Responder};
use entity::entity::prelude::Family;
use entity::sea_orm::ActiveValue::NotSet;
use entity::sea_orm::{ActiveModelTrait, EntityTrait, QuerySelect};
use entity::{entity::family, sea_orm::Set};

use crate::AppState;

#[post("/family")]
async fn post_family(info: web::Json<family::Model>, state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let family_model = family::ActiveModel {
        id: NotSet,
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
        Ok(model) => Either::Left(HttpResponse::Created().json(model)),
        Err(error) => Either::Right(HttpResponse::Conflict().json(error.to_string())),
    }
}

#[post("/family/{id}")]
async fn update_family(
    id: web::Path<i32>,
    info: web::Json<family::Model>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let family = Family::find_by_id(id.into_inner()).one(db_connection).await;
    if let Ok(family) = family {
        if let Some(family) = family {
            let mut family_model: family::ActiveModel = family.into();
            family_model.father_name = Set(info.father_name.to_owned());
            family_model.mother_name = Set(info.mother_name.to_owned());
            family_model.father_occupation_id = Set(info.father_occupation_id.to_owned());
            family_model.mother_occupation_id = Set(info.mother_occupation_id.to_owned());
            family_model.address = Set(info.address.to_owned());
            family_model.city = Set(info.city.to_owned());
            family_model.state = Set(info.state.to_owned());
            family_model.zip = Set(info.zip.to_owned());
            family_model.tenth_year = Set(info.tenth_year.to_owned());
            family_model.tenth_percentage = Set(info.tenth_percentage.to_owned());
            family_model.tenth_board_id = Set(info.tenth_board_id.to_owned());
            family_model.twelfth_year = Set(info.twelfth_year.to_owned());
            family_model.twelfth_percentage = Set(info.twelfth_percentage.to_owned());
            family_model.twelfth_board_id = Set(info.twelfth_board_id.to_owned());
            let result = family_model.update(db_connection).await;
            match result {
                Ok(model) => return HttpResponse::NoContent().json(model),
                Err(error) => return HttpResponse::Conflict().json(error.to_string()),
            };
        } else {
            return HttpResponse::NotFound().json("family not found");
        }
    } else {
        return HttpResponse::NotFound().json("family not found");
    }
}

#[get("/tenth_year")]
async fn get_distinct_tenth_years(state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let tenth_years = 
    Family::find().select_only()
        .column(family::Column::TenthYear)
        .group_by(family::Column::TenthYear)
        .into_json()
        .all(db_connection)
        .await;

    match tenth_years {
        Ok(years) => return HttpResponse::Ok().json(years),
        Err(error) => return HttpResponse::BadRequest().json(error.to_string()),
    }
}

#[get("/twelfth_year")]
async fn get_distinct_twelfth_years(state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let twelfth_years = 
    Family::find()
        .select_only()
        .column(family::Column::TwelfthYear)
        .group_by(family::Column::TwelfthYear)
        .into_json()
        .all(db_connection)
        .await;

    match twelfth_years {
        Ok(years) => return HttpResponse::Ok().json(years),
        Err(error) => return HttpResponse::BadRequest().json(error.to_string()),
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

#[delete("/family/{id}")]
async fn delete_family_by_id(
    family_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let family = Family::delete_by_id(family_id.to_owned())
        .exec(db_connection)
        .await;

    match family {
        Ok(family) => Either::Left(HttpResponse::Ok().json(family.rows_affected)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(post_family)
        .service(update_family)
        .service(get_distinct_tenth_years)
        .service(get_distinct_twelfth_years)
        .service(get_familys)
        .service(get_family_by_id)
        .service(delete_family_by_id);
}
