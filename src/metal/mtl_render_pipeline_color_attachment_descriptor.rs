use crate::{
  obj::Object,
  runtime::NSObjectBase,
  PrivateMarker,
  metal::*
};

pub struct MTLRenderPipelineColorAttachmentDescriptor(PrivateMarker);

impl Object for MTLRenderPipelineColorAttachmentDescriptor {}
unsafe impl objc::Message for MTLRenderPipelineColorAttachmentDescriptor {}

impl NSObjectBase for MTLRenderPipelineColorAttachmentDescriptor {
  fn class_name() -> &'static str { "MTLRenderPipelineColorAttachmentDescriptor" }
}

impl MTLRenderPipelineColorAttachmentDescriptor {
  pub unsafe fn set_pixel_format(&self, pixel_format: MTLPixelFormat) {
    msg_send![self, setPixelFormat: pixel_format]
  }
}
