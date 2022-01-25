use crate::{
  obj::Object,
  PrivateMarker,
  runtime::NSObjectBase,
  foundation::NSString,
  metal::*
};

pub struct MTLComputePipelineDescriptor(PrivateMarker);

impl Object for MTLComputePipelineDescriptor {}
unsafe impl objc::Message for MTLComputePipelineDescriptor {}

impl NSObjectBase for MTLComputePipelineDescriptor {
  fn class_name() -> &'static str { "MTLComputePipelineDescriptor" }
}

impl MTLComputePipelineDescriptor {
  pub unsafe fn set_label(&self, label: *mut NSString) {
    msg_send![self, setLabel:label]
  }

  pub unsafe fn set_compute_function(&self, func: *mut MTLFunction) {
    msg_send![self, setComputeFunction:func]
  }
}
