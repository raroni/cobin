use crate::{
  obj::Object,
  PrivateMarker,
  runtime::NSObjectBase
};

pub trait MTLAccelerationStructureDescriptorBase {}

pub struct MTLAccelerationStructureDescriptor(PrivateMarker);

unsafe impl objc::Message for MTLAccelerationStructureDescriptor {}
impl Object for MTLAccelerationStructureDescriptor {}

impl NSObjectBase for MTLAccelerationStructureDescriptor {
  fn class_name() -> &'static str { "MTLAccelerationStructureDescriptor" }
}

impl MTLAccelerationStructureDescriptorBase for MTLAccelerationStructureDescriptor { }
