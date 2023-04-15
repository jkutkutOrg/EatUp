use rocket::{State};
use tokio_postgres::{Client};
use uuid::Uuid;
use rocket::http::{Status};

use crate::qr;
use crate::route_tools::InvalidAPI;
use crate::tools::UuidWrapper;
use crate::api::*;

mod model;
mod product;
mod session;
mod order;

// TODO refactor get_* with generics

pub use model::*;
pub use product::*;
pub use session::*;
pub use order::*;
