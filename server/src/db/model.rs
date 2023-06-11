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
  allergies: Vec<Allergy>,
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
  name: String
}

impl Allergy {
  pub fn new(
    id: Uuid,
    name: String,
  ) -> Self {
    Self {
      id,
      name,
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