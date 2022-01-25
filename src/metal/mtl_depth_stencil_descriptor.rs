use crate::{
  obj::Object,
  PrivateMarker,
  runtime::NSObjectBase,
  metal::*
};

pub struct MTLDepthStencilDescriptor(PrivateMarker);

impl Object for MTLDepthStencilDescriptor {}
unsafe impl objc::Message for MTLDepthStencilDescriptor {}

impl NSObjectBase for MTLDepthStencilDescriptor {
  fn class_name() -> &'static str { "MTLDepthStencilDescriptor" }
}

impl MTLDepthStencilDescriptor {
  pub unsafe fn set_depth_compare_function(&self, function: MTLCompareFunction) {
    msg_send![self, setDepthCompareFunction:function]
  }

  pub unsafe fn set_depth_write_enabled(&self, enabled: bool) {
    let enabled_native: objc::runtime::BOOL = enabled.into();
    msg_send![self, setDepthWriteEnabled:enabled_native]
  }
}
