use actix_web::{get, patch, post, web::Json, web::Path, App, HttpResponse, HttpServer, Responder};
use validator::Validate;

mod models;
use crate::models::pizza::{BuyPizzaRequest, UpdatePizzaRequest};

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Getting pizzas!")
}

#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            HttpResponse::Ok().body(format!("Buying pizza: {}", pizza_name))
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
    HttpServer::new(|| {
        App::new()
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
