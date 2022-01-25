use crate::{
  PrivateMarker,
  obj::Object
};

pub struct MTLDepthStencilState(PrivateMarker);

impl Object for MTLDepthStencilState {}
unsafe impl objc::Message for MTLDepthStencilState {}
