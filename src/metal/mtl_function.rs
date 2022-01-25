use crate::{
  PrivateMarker,
  obj::Object
};

pub struct MTLFunction(PrivateMarker);

impl Object for MTLFunction {}
unsafe impl objc::Message for MTLFunction {}
