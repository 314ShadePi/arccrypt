use dyn_clonable::*;
use mopa::mopafy;
use std::fmt::Debug;

#[clonable]
#[typetag::serde(tag = "type")]
pub trait Payload: Debug + Clone + mopa::Any {}

mopafy!(Payload);
