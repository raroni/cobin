use crate::{
  obj::Object,
  runtime::NSObjectBase,
  PrivateMarker,
  metal::*
};

pub struct MTLRenderPassDescriptor(PrivateMarker);

impl Object for MTLRenderPassDescriptor {}
unsafe impl objc::Message for MTLRenderPassDescriptor {}

impl NSObjectBase for MTLRenderPassDescriptor {
  fn class_name() -> &'static str { "MTLRenderPassDescriptor" }
}

impl MTLRenderPassDescriptor {
  pub unsafe fn color_attachments(&self) -> *mut MTLRenderPassColorAttachmentDescriptorArray {
    msg_send![self, colorAttachments]
  }

  pub unsafe fn depth_attachment(&self) -> *mut MTLRenderPassDepthAttachmentDescriptor {
    msg_send![self, depthAttachment]
  }
}
