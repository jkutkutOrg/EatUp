use std::path::Path;
use std::process::Command;
use std::{fs, io};

use crate::model::*;
use crate::{MICROSERVICES, ENV};
use crate::api::error::InvalidAPI;
use crate::dotenv;

mod generic;
mod servicehandler;
mod status;

pub use generic::*;
pub use servicehandler::*;
pub use status::*;

// TODO run server when installing