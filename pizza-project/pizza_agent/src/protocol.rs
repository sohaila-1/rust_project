use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Request {
    ListRecipes,
    Order {
        recipe_name: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Response {
    Recipes(Vec<String>),

    ProductionError {
        order_id: Option<u32>,
        error: String,
    },
}