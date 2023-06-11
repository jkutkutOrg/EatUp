use super::*;

#[get("/orders/<session_id>")]
pub(super) async fn orders(
  db: &State<Client>,
  session_id: UuidWrapper
) -> Result<Json<Vec<db::Order>>, InvalidAPI> {
  let session_id = match session_id.get() {
    Ok(session_id) => session_id,
    Err(_) => return Err(InvalidAPI::new(ERROR_INVALID_SESSION_ID))
  };

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
  let session_id = match session_id.get() {
    Ok(session_id) => session_id,
    Err(_) => return Err(InvalidAPI::new(ERROR_INVALID_SESSION_ID))
  };
  let order = order.into_inner();
  match db::create_order(db, session_id, order).await {
    Err(e) => Err(e),
    Ok(_) => Ok(Json("Order created"))
  }
}