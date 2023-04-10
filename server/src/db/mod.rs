use rocket::{State};
use tokio_postgres::{Client};
use uuid::Uuid;
use rocket::http::{Status};
use serde::Serialize;

use crate::api::{ProductQuery, SessionQuery};

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

// TODO refactor get_* with generics

pub async fn get_products(
    db: &State<Client>,
    filters: ProductQuery
) -> Result<Vec<Product>, Status> {
    let mut query: String = "SELECT p.* FROM product p".to_string();
    
    // TODO check if the filters are valid

    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();

    let allergies: bool = filters.allergies.len() > 0;
    let categories: bool = filters.categories.len() > 0;
    if allergies || categories {
        let mut count = 1;
        if categories {
            query.push_str(", product_category pc, category c");
        }
        query.push_str("\nWHERE\n");
        if categories {
            query.push_str("\np.id = pc.product_id and c.id = pc.category_id");
            query.push_str(format!(" and c.name = ANY(${})", count).as_str());
            params.push(&filters.categories);
            count += 1;
        }
        if allergies {
            if categories {
                query.push_str(" and ");
            }
            query.push_str(format!(
                "p.id not in (
                    SELECT prod.id
                    FROM product prod
                    JOIN product_allergy pa ON prod.id = pa.product_id
                    JOIN allergy a ON a.id = pa.allergy_id
                    WHERE a.name IN (
                      SELECT unnest(${}::text[])
                    )
                )", count
            ).as_str());
            params.push(&filters.allergies);
            //count += 1;
        }
    }

    let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &params;

    let mut products: Vec<Product> = Vec::new();
    let stmt = db.prepare(&query).await.unwrap();
    for row in db.query(&stmt, params).await.unwrap() {
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

// ---------------------------------------------

#[derive(Debug, Serialize)]
pub struct Session {
}

pub async fn get_sessions(
    db: &State<Client>,
    filters: SessionQuery
) -> Result<Vec<Session>, Status> {
    Err(Status::NotImplemented)
}