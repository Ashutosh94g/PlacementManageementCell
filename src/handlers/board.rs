use actix_web::{delete, get, post, web, Either, HttpResponse, Responder};
use entity::entity::prelude::Board;
use entity::sea_orm::ActiveValue::NotSet;
use entity::sea_orm::{ActiveModelTrait, EntityTrait};
use entity::{entity::board, sea_orm::Set};

use crate::AppState;

#[post("/board")]
async fn post_board(info: web::Json<board::Model>, state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let board_model = board::ActiveModel {
        id: NotSet,
        value: Set(info.value.to_owned()),
    };

    let result = Board::insert(board_model)
        .exec_with_returning(db_connection)
        .await;

    match result {
        Ok(model) => Either::Left(HttpResponse::Ok().json(model)),
        Err(error) => Either::Right(HttpResponse::InternalServerError().json(error.to_string())),
    }
}

#[post("/board/{id}")]
async fn update_board(
    id: web::Path<i32>,
    info: web::Json<board::Model>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let board = Board::find_by_id(id.into_inner()).one(db_connection).await;
    if let Ok(board) = board {
        if let Some(board) = board {
            let mut board_model: board::ActiveModel = board.into();
            board_model.value = Set(info.value.to_owned());
            let result = board_model.update(db_connection).await;
            match result {
                Ok(model) => return HttpResponse::Ok().json(model),
                Err(error) => return HttpResponse::InternalServerError().json(error.to_string()),
            };
        } else {
            return HttpResponse::NotFound().json("Board not found");
        }
    } else {
        return HttpResponse::NotFound().json("Board not found");
    }
}

#[get("/board")]
async fn get_boards(state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let boards = Board::find().all(db_connection).await;

    match boards {
        Ok(boards) => Either::Left(HttpResponse::Ok().json(boards)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

#[get("/board/{id}")]
async fn get_board_by_id(board_id: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let db_connection = &state.db_connection;
    let board = Board::find_by_id(board_id.to_owned())
        .one(db_connection)
        .await;

    match board {
        Ok(board) => Either::Left(HttpResponse::Ok().json(board)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

#[delete("/board/{id}")]
async fn delete_board_by_id(
    board_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &state.db_connection;
    let board = Board::delete_by_id(board_id.to_owned())
        .exec(db_connection)
        .await;

    match board {
        Ok(board) => Either::Left(HttpResponse::Ok().json(board.rows_affected)),
        Err(error) => Either::Right(HttpResponse::BadRequest().json(error.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(post_board)
        .service(get_boards)
        .service(update_board)
        .service(get_board_by_id)
        .service(delete_board_by_id);
}
