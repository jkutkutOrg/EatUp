use super::*;

pub async fn get_orders(
    db: &State<Client>,
    session_id: Uuid,
) -> Result<Vec<Order>, InvalidAPI> {
    // ? Idea check if session exists
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
    session_id: Uuid,
    order: OrderQuery
) -> Result<(), InvalidAPI> {
    if order.products.len() == 0 {
        return Err(InvalidAPI::new(ERROR_NO_PRODUCTS_ORDER));
    }
    if !is_active_session(db, session_id).await {
        return Err(InvalidAPI::new(ERROR_INVALID_SESSION_ID));
    }
    let query: String = "
        INSERT INTO orders (session)
        VALUES ($1) RETURNING id
    ".to_string();
    let stmt = db.prepare(&query).await.unwrap();
    match db.query_one(&stmt, &[&session_id]).await {
        Ok(row) => {
            let order_id: Uuid = row.get(0);
            for product in &order.products {
                match create_product_order(db, order_id, product).await {
                    Err(e) => {
                        cancel_order(db, order_id).await;
                        return Err(e);
                    },
                    _ => ()
                }
            }
            Ok(())
        },
        Err(_) => Err(InvalidAPI::new(ERROR_INVALID_SESSION_ID))
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
        Err(_) => Err(InvalidAPI::new(ERROR_INVALID_PRODUCT_ID))
    }
}

async fn cancel_order(
    db: &State<Client>,
    order_id: Uuid
) {
    let query: String = "DELETE FROM orders WHERE id = $1".to_string();
    let stmt = db.prepare(&query).await.unwrap();
    db.execute(&stmt, &[&order_id]).await.unwrap();
}