use rocket::{get};
use rocket::http::{Status};

use super::ProductQuery;

#[get("/products?<filters..>")]
pub(super) fn products(filters: ProductQuery) -> Result<String, Status> {
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

    // Ok(s)
    return Err(Status::NotImplemented);
}
