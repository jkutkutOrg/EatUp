use rocket::{State};
use tokio_postgres::{Client};
use uuid::Uuid;
use rocket::http::{Status};
use serde::Serialize;

use crate::api::ProductQuery;

#[derive(Debug, Serialize)]
struct Allergy {
    id: Uuid,
    name: String,
    img_id: String
}

#[derive(Debug, Serialize)]
struct ProductCategory {
    id: Uuid,
    name: String
}

#[derive(Debug, Serialize)]
pub struct Product {
    id: Uuid,
    name: String,
    description: String,
    img_id: String,
    price: f32,
    allergies: Vec<Allergy>,
    categories: Vec<ProductCategory>
}

pub async fn get_products(
    db: &State<Client>,
    filters: ProductQuery
) -> Result<Vec<Product>, Status> {
    let query: String = "SELECT * FROM product".to_string();
    // TODO filters

    let mut products: Vec<Product> = Vec::new();
    let stmt = db.prepare(&query).await.unwrap();
    for row in db.query(&stmt, &[]).await.unwrap() {
        let product = Product {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            img_id: row.get(3),
            price: row.get(4),
            allergies: get_allergies(db, row.get(0)).await,
            categories: get_product_categories(db, row.get(0)).await
        };
        products.push(product);
    }
    Ok(products)
}

async fn get_allergies(
    db: &State<Client>,
    product_id: Uuid
) -> Vec<Allergy> {
    let query: String = "
        SELECT allergy.* FROM allergy, product_allergy
        WHERE allergy.id = product_allergy.allergy_id and product_allergy.product_id = $1
    ".to_string();
    
    let mut allergies: Vec<Allergy> = Vec::new();
    let stmt = db.prepare(&query).await.unwrap();
    for row in db.query(&stmt, &[&product_id]).await.unwrap() {
        let allergy = Allergy {
            id: row.get(0),
            name: row.get(1),
            img_id: row.get(2)
        };
        allergies.push(allergy);
    }
    allergies
}

async fn get_product_categories(
    db: &State<Client>,
    product_id: Uuid
) -> Vec<ProductCategory> {
    let query: String = "
        SELECT category.* FROM product_category pc, category
        WHERE pc.category_id = category.id and pc.product_id = $1
    ".to_string();

    let mut categories: Vec<ProductCategory> = Vec::new();
    let stmt = db.prepare(&query).await.unwrap();
    for row in db.query(&stmt, &[&product_id]).await.unwrap() {
        let category = ProductCategory {
            id: row.get(0),
            name: row.get(1)
        };
        categories.push(category);
    }
    categories
}