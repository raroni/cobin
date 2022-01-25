use crate::{
  obj::Object,
  PrivateMarker,
  runtime::NSObjectBase,
  metal::*,
  metal::mtl_render_pass_attachment_descriptor::MTLRenderPassAttachmentDescriptorBase
};

pub struct MTLRenderPassColorAttachmentDescriptor(PrivateMarker);

impl Object for MTLRenderPassColorAttachmentDescriptor {}
unsafe impl objc::Message for MTLRenderPassColorAttachmentDescriptor {}

impl NSObjectBase for MTLRenderPassColorAttachmentDescriptor {
  fn class_name() -> &'static str { "MTLRenderPassColorAttachmentDescriptor" }
}

impl MTLRenderPassAttachmentDescriptorBase for MTLRenderPassColorAttachmentDescriptor {}

impl MTLRenderPassColorAttachmentDescriptor {
  pub unsafe fn set_clear_color(&self, clear_color: MTLClearColor) {
    msg_send![self, setClearColor: clear_color]
  }
}
