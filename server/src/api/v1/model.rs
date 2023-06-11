use rocket::FromForm;
use rocket::serde::Deserialize;
use crate::db::Product;

#[derive(Deserialize, FromForm)]
pub struct ProductQuery {
  #[field(name = "category")]
  pub categories: Vec<String>,
  #[field(name = "allergy")]
  pub allergies: Vec<String>,
}

#[derive(Deserialize, FromForm)]
pub struct SessionQuery {
  #[field(name = "table_id")]
  pub table_ids: Vec<String>,
  pub in_progress: Option<bool>,
}

#[derive(Deserialize)]
pub struct OrderQuery {
  pub products: Vec<ProductOrderQuery>
}

#[derive(Deserialize)]
pub struct ProductOrderQuery {
  pub quantity: i32,
  pub product: Product
}