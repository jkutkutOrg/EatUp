use rocket::{State};
use tokio_postgres::{Client};
use rocket::http::{Status};
use serde::Serialize;

use crate::api::ProductQuery;

#[derive(Debug, Serialize)]
pub struct Product {

}

pub fn get_products(
    db: &State<Client>,
    filters: ProductQuery
) -> Result<Vec<Product>, Status> {

    let mut s: String = "v1 product".to_string();

    if filters.categories.len() > 1 {
        s.push_str(&format!(" with categories {:?}", filters.categories));
    } else if let Some(category) = filters.categories.get(0) {
        s.push_str(&format!(" with category {:?}", category));
    }
    else {
        s.push_str(" with no category");
    }

    if filters.allergies.len() > 1 {
        s.push_str(&format!(" with allergies {:?}", filters.allergies));
    } else if let Some(allergy) = filters.allergies.get(0) {
        s.push_str(&format!(" with allergy {:?}", allergy));
    }
    else {
        s.push_str(" with no allergy");
    }

    println!("get_products: {}", s);

    // let mut products: Vec<Product> = Vec::new();

    Err(Status::NotImplemented)
}