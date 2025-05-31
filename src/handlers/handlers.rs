
use crate::db::Database;
use actix_web::{web, HttpResponse, Responder, ResponseError};
use crate::models::route::Route;

pub async fn create_route(
    db: web::Data<Database>,
    route: web::Json<Route>,
) -> impl Responder {
    match db.create_route(&route) {
        Ok(id) => HttpResponse::Created().json(id),
        Err(e) => e.error_response(),
    }
}

pub async fn get_route(
    db: web::Data<Database>,
    id: web::Path<i64>,
) -> impl Responder {
    match db.get_route(*id) {
        Ok(route) => HttpResponse::Ok().json(route),
        Err(e) => e.error_response(),
    }
}

pub async fn get_all_routes(db: web::Data<Database>) -> impl Responder {
    match db.get_all_routes() {
        Ok(routes) => HttpResponse::Ok().json(routes),
        Err(e) => e.error_response(),
    }
}

pub async fn update_route(
    db: web::Data<Database>,
    id: web::Path<i64>,
    route: web::Json<Route>,
) -> impl Responder {
    match db.update_route(*id, &route) {
        Ok(true) => HttpResponse::Ok().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(e) => e.error_response(),
    }
}

pub async fn delete_route(
    db: web::Data<Database>,
    id: web::Path<i64>,
) -> impl Responder {
    match db.delete_route(*id) {
        Ok(true) => HttpResponse::Ok().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(e) => e.error_response(),
    }
}