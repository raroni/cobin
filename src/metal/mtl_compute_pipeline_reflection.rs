use crate::{
  obj::Object,
  PrivateMarker,
  runtime::NSObjectBase
};

pub struct MTLComputePipelineReflection(PrivateMarker);

impl Object for MTLComputePipelineReflection {}
unsafe impl objc::Message for MTLComputePipelineReflection {}

impl NSObjectBase for MTLComputePipelineReflection {
  fn class_name() -> &'static str { "MTLComputePipelineReflection" }
}
