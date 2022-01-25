use crate::{
  obj::Object,
  PrivateMarker,
  runtime::NSObjectBase,
  metal::mtl_render_pass_attachment_descriptor::MTLRenderPassAttachmentDescriptorBase
};

pub struct MTLRenderPassDepthAttachmentDescriptor(PrivateMarker);

impl Object for MTLRenderPassDepthAttachmentDescriptor {}
unsafe impl objc::Message for MTLRenderPassDepthAttachmentDescriptor {}

impl NSObjectBase for MTLRenderPassDepthAttachmentDescriptor {
  fn class_name() -> &'static str { "MTLRenderPassDepthAttachmentDescriptor" }
}

impl MTLRenderPassAttachmentDescriptorBase for MTLRenderPassDepthAttachmentDescriptor { }

impl MTLRenderPassDepthAttachmentDescriptor {
  pub unsafe fn set_clear_depth(&self, depth: f64) {
    msg_send![self, setClearDepth: depth]
  }
}
