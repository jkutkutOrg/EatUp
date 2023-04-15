use rocket::{State};
use tokio_postgres::{Client};
use uuid::Uuid;
use rocket::http::{Status};

use crate::api::{ProductQuery, SessionQuery, OrderQuery, ProductOrderQuery};
use crate::qr;
use crate::route_tools::InvalidAPI;
use crate::tools::UuidWrapper;

pub use model::*;

pub mod model {
    use uuid::Uuid;
    use serde::{Serialize, Deserialize};

    // ******* Product *******

    #[derive(Serialize, Deserialize)]
    pub struct Product {
        pub id: Uuid,
        name: String,
        description: String,
        img_id: String,
        price: f32,
        allergies: Vec<Allergy>, // ? TODO allergies or allergens?
        categories: Vec<ProductCategory>
    }

    impl Product {
        pub fn new(
            id: Uuid,
            name: String,
            description: String,
            img_id: String,
            price: f32,
            allergies: Vec<Allergy>,
            categories: Vec<ProductCategory>
        ) -> Self {
            Self {
                id,
                name,
                description,
                img_id,
                price,
                allergies,
                categories
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct Allergy {
        id: Uuid,
        name: String,
        img_id: String
    }

    impl Allergy {
        pub fn new(
            id: Uuid,
            name: String,
            img_id: String
        ) -> Self {
            Self {
                id,
                name,
                img_id
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct ProductCategory {
        id: Uuid,
        name: String
    }

    impl ProductCategory {
        pub fn new(
            id: Uuid,
            name: String
        ) -> Self {
            Self {
                id,
                name
            }
        }
    }

    // ******* Session *******

    #[derive(Serialize)]
    pub struct Session {
        id: Uuid,
        table_id: String,
        in_progress: bool
    }

    impl Session {
        pub fn new(
            id: Uuid,
            table_id: String,
            in_progress: bool
        ) -> Self {
            Self {
                id,
                table_id,
                in_progress
            }
        }
    }

    #[derive(Serialize)]
    pub struct SessionUuid {
        simple_id: String,
        id: Uuid,
        qr_img: String
    }

    impl SessionUuid {
        pub fn new(
            simple_id: String,
            id: Uuid,
            qr_img: String
        ) -> Self {
            Self {
                simple_id,
                id,
                qr_img
            }
        }
    }

    // ******* Order *******

    #[derive(Serialize)]
    pub struct Order {
        id: Uuid,
        products: Vec<ProductOrder>
    }

    impl Order {
        pub fn new(
            id: Uuid,
            products: Vec<ProductOrder>
        ) -> Self {
            Self {
                id,
                products
            }
        }
    }

    #[derive(Serialize)]
    pub struct ProductOrder {
        id: Uuid,
        quantity: i32,
        product: Product
    }

    impl ProductOrder {
        pub fn new(
            id: Uuid,
            quantity: i32,
            product: Product
        ) -> Self {
            Self {
                id,
                quantity,
                product
            }
        }
    }
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
        products.push(Product::new(
            row.get(0), // id
            row.get(1), // name
            row.get(2), // description
            row.get(3), // img_id
            row.get(4), // price
            get_allergies(db, row.get(0)).await, // allergies
            get_product_categories(db, row.get(0)).await // categories
        ));
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
        allergies.push(Allergy::new(
            row.get(0), // id
            row.get(1), // name
            row.get(2) // img_id
        ));
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
        categories.push(ProductCategory::new(
            row.get(0), // id
            row.get(1) // name
        ));
    }
    categories
}

// ---------------------------------------------

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
        sessions.push(Session::new(
            row.get(0), // id
            row.get(1), // table_id
            row.get(2) // in_progress
        ));
    }
    Ok(sessions)
}

pub async fn create_session(
    db: &State<Client>,
    table_id: String
) -> Result<SessionUuid, InvalidAPI> {
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
            Ok(SessionUuid::new(
                simple_id,
                id,
                qr_path // qr_img
            ))
        },
        Err(_) => Err(InvalidAPI::new(
            format!("There is already a session in progress for table {table_id}")
        ))
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

// ---------------------------------------------

pub async fn get_orders(
    db: &State<Client>,
    session_id: UuidWrapper
) -> Result<Vec<Order>, Status> {
    let session_id: Uuid = session_id.unwrap();
    let query = "SELECT * FROM orders WHERE session = $1".to_string();
    let stmt = db.prepare(&query).await.unwrap();
    let mut orders: Vec<Order> = Vec::new();
    for row in db.query(&stmt, &[&session_id]).await.unwrap() {
        orders.push(Order::new(
            row.get(0), // id
            get_product_order(db, row.get(0)).await // products
        ));
    }
    Ok(orders)
}

async fn get_product_order(
    db: &State<Client>,
    order_id: Uuid
) -> Vec<ProductOrder> {
    let query = "
        SELECT po.*, p.*
        FROM product_order po, product p
        WHERE
            po.product_id = p.id and
            po.order_id = $1
    ".to_string();

    let mut products: Vec<ProductOrder> = Vec::new();
    let stmt = db.prepare(&query).await.unwrap();
    for row in db.query(&stmt, &[&order_id]).await.unwrap() {
        let uuid: Uuid = row.get(3);
        let product = Product::new(
            uuid, // id
            row.get(4), // name
            row.get(5), // description
            row.get(6), // img_id
            row.get(7), // price
            get_allergies(db, uuid).await, // allergies
            get_product_categories(db, uuid).await // categories
        );
        products.push(ProductOrder::new(
            row.get(0), // id
            row.get(2), // quantity
            product
        ));
    }
    products
}

pub async fn create_order(
    db: &State<Client>,
    session_id: UuidWrapper,
    order: OrderQuery
) -> Result<(), InvalidAPI> {
    let query: String = "
        INSERT INTO orders (session)
        VALUES ($1) RETURNING id
    ".to_string();
    let stmt = db.prepare(&query).await.unwrap();
    match db.query_one(&stmt, &[&session_id.unwrap()]).await {
        Ok(row) => {
            let order_id: Uuid = row.get(0);
            for product in &order.products {
                match create_product_order(db, order_id, product).await {
                    Err(e) => return Err(e),
                    _ => ()
                }
            }
            Ok(())
        },
        Err(_) => Err(InvalidAPI::new(format!("Invalid session id")))
    }
}

async fn create_product_order(
    db: &State<Client>,
    order_id: Uuid,
    product: &ProductOrderQuery
) -> Result<(), InvalidAPI> {
    let query: String = "
        INSERT INTO product_order (order_id, product_id, quantity)
        VALUES ($1, $2, $3)
    ".to_string();
    let stmt = db.prepare(&query).await.unwrap();
    let params: [&(dyn tokio_postgres::types::ToSql + Sync); 3] = [
        &order_id,
        &product.product.id,
        &product.quantity
    ];
    match db.execute(&stmt, &params).await {
        Ok(_) => Ok(()),
        Err(_) => Err(InvalidAPI::new(format!("Invalid product id")))
    }
}
