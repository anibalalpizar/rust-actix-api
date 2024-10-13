use log::error;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use crate::models::pizza::Pizza;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        log::info!("Trying to connect to SurrealDB server");
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        log::info!("Connected to SurrealDB server");

        client.use_ns("namespace1").use_db("pizzashop").await?;

        Ok(Database {
            client,
            name_space: String::from("namespace1"),
            db_name: String::from("pizzashop"),
        })
    }

    pub async fn get_pizzas(&self) -> Option<Vec<Pizza>> {
        let result = self.client.select("pizzas").await;
        match result {
            Ok(pizzas) => Some(pizzas),
            Err(_) => None,
        }
    }

    pub async fn add_pizza(&self, pizza_name: String) -> Option<Pizza> {
        let pizza = Pizza { pizza_name };
        log::info!("Adding pizza: {:?}", pizza);
        let created_pizza = self.client.create("pizzas").content(pizza).await;
        match created_pizza {
            Ok(created) => created,
            Err(e) => {
                error!("Failed to create pizza: {}", e);
                None
            }
        }
    }
}
