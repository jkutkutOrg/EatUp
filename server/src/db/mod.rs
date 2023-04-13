use rocket::{State};
use tokio_postgres::{Client};
use uuid::Uuid;
use rocket::http::{Status};
use serde::Serialize;

use crate::api::{ProductQuery, SessionQuery};
use crate::qr;
use crate::tools::UuidWrapper;

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
        }
    }

    let mut products: Vec<Product> = Vec::new();
    let stmt = db.prepare(&query).await.unwrap();
    for row in db.query(&stmt, &params).await.unwrap() {
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
    id: Uuid,
    table_id: String,
    in_progress: bool
}

pub async fn get_sessions(
    db: &State<Client>,
    filters: SessionQuery
) -> Result<Vec<Session>, Status> {
    let mut query: String = "SELECT * FROM session".to_string();
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
    let mut count = 1;

    if filters.table_ids.len() > 0 {
        query.push_str(format!(" WHERE table_id = ANY(${})", count).as_str());
        params.push(&filters.table_ids);
        count += 1;
    }
    if filters.in_progress.is_some() {
        if count > 1 {
            query.push_str(" and ");
        } else {
            query.push_str(" WHERE ");
        }
        query.push_str(format!(" in_progress = ${}", count).as_str());
        params.push(&filters.in_progress);
    }

    let mut sessions: Vec<Session> = Vec::new();
    let stmt = db.prepare(&query).await.unwrap();
    for row in db.query(&stmt, &params).await.unwrap() {
        let session = Session {
            id: row.get(0),
            table_id: row.get(1),
            in_progress: row.get(2)
        };
        sessions.push(session);
    }
    Ok(sessions)
}

#[derive(Debug, Serialize)]
pub struct SessionMap { // TODO find a better name
    simple_id: String,
    id: Uuid,
    qr_img: String
}

pub async fn create_session(
    db: &State<Client>,
    table_id: String
) -> Result<SessionMap, Status> {
    let query: String = "SELECT * FROM create_session($1)".to_string();
    let stmt = db.prepare(&query).await.unwrap();
    match db.query_one(&stmt, &[&table_id]).await {
        Ok(row) => {
            let simple_id: String = row.get(0);
            let id: Uuid = row.get(1);
            let id_str = id.to_string();

            let qr_path = format!("/qr/{}.png", &id_str);
            let qr_real_path = format!("/db/public{}", &qr_path);

            qr::generate_with_debug(&id_str, &qr_real_path);
            Ok(SessionMap {
                simple_id,
                id,
                qr_img: qr_path
            })
        },
        Err(e) => {
            println!("Error: {}", e); // TODO handle error
            return Err(Status::InternalServerError);
        }
    }
}

pub async fn end_session(
    db: &State<Client>,
    session_id: UuidWrapper
) -> Result<(), Status> {
    let session_id: Uuid = session_id.unwrap();
    let query: String = "UPDATE session SET in_progress = false WHERE id = $1;".to_string();
    let stmt = db.prepare(&query).await.unwrap();
    db.execute(&stmt, &[&session_id]).await.unwrap();
    let file = format!("/db/public/qr/{}.png", session_id.to_string());
    std::fs::remove_file(file).unwrap();
    Ok(())
}