use crate::{
  obj::Object,
  PrivateMarker
};

pub struct MTLRenderPipelineState(PrivateMarker);

impl Object for MTLRenderPipelineState {}
unsafe impl objc::Message for MTLRenderPipelineState {}
