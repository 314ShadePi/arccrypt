pub mod prelude;
pub mod traits;
use crate::prelude::Payload;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Payload)]
pub struct Tester {}
