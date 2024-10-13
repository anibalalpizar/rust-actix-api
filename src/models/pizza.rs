use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct BuyPizzaRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub pizza_name: String,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdatePizzaRequest {
    pub id: String,
}

#[derive(Validate, Deserialize, Serialize, Debug)]
pub struct Pizza {
    pub pizza_name: String,
}

impl Pizza {
    pub fn new(pizza_name: String) -> Pizza {
        Pizza { pizza_name }
    }
}
