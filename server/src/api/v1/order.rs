use super::*;

#[get("/orders/<session_id>")]
pub(super) async fn orders(
    db: &State<Client>,
    session_id: UuidWrapper
) -> Result<Json<Vec<db::Order>>, Status> {
    match db::get_orders(db, session_id).await {
        Err(e) => Err(e),
        Ok(orders) => Ok(Json(orders))
    }
}

#[post("/orders/<session_id>", data = "<order>")]
pub(super) async fn create_order(
    db: &State<Client>,
    session_id: UuidWrapper,
    order: Json<OrderQuery>
) -> Result<Json<&'static str>, InvalidAPI> {
    let order = order.into_inner();
    match db::create_order(db, session_id, order).await {
        Err(e) => Err(e),
        Ok(_) => Ok(Json("Order created"))
    }
}