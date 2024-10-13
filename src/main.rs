use std::env;

use actix_web::web::Data;
use actix_web::{get, patch, post, web::Json, web::Path, App, HttpResponse, HttpServer, Responder};
use env_logger;
use validator::Validate;

mod db;
mod models;

use crate::db::Database;
use crate::models::pizza::{BuyPizzaRequest, UpdatePizzaRequest};

#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> impl Responder {
    let pizzas = db.get_pizzas().await;
    match pizzas {
        Some(pizzas) => HttpResponse::Ok().json(pizzas),
        None => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>, db: Data<Database>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();

            let new_pizza = db.add_pizza(pizza_name).await;

            match new_pizza {
                Some(pizza) => {
                    HttpResponse::Ok().body(format!("Bought pizza: {}", pizza.pizza_name))
                }
                None => HttpResponse::Ok().body("Failed to buy pizza"),
            }
        }
        Err(e) => HttpResponse::BadRequest().body(format!("Validation error: {}", e)),
    }
}

#[patch("/updatepizza/{id}")]
async fn update_pizza(params: Path<UpdatePizzaRequest>) -> impl Responder {
    let id = params.id.clone();
    HttpResponse::Ok().body(format!("Updating pizza with id: {}", id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let _db = Database::init()
        .await
        .expect("Failed to initialize database");
    let db_data = Data::new(_db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
