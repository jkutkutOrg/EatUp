use super::*;

#[get("/products?<filters..>")]
pub(super) async fn products(
  db: &State<Client>,
  filters: ProductQuery
) -> Result<Json<Vec<db::Product>>, Status> {
  match db::get_products(db, filters).await {
    Err(e) => Err(e),
    Ok(products) => Ok(Json(products))
  }
}