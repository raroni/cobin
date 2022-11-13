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

  pub unsafe fn set_alpha_blend_operation(&self, operation: MTLBlendOperation) {
    msg_send![self, setAlphaBlendOperation:operation]
  }

  pub unsafe fn set_source_rgb_blend_factor(&self, factor: MTLBlendFactor) {
    msg_send![self, setSourceRGBBlendFactor:factor]
  }

  pub unsafe fn set_destination_rgb_blend_factor(&self, factor: MTLBlendFactor) {
    msg_send![self, setDestinationRGBBlendFactor:factor]
  }

  pub unsafe fn set_source_alpha_blend_factor(&self, factor: MTLBlendFactor) {
    msg_send![self, setSourceAlphaBlendFactor:factor]
  }

  pub unsafe fn set_destination_alpha_blend_factor(&self, factor: MTLBlendFactor) {
    msg_send![self, setDestinationAlphaBlendFactor:factor]
  }

  pub unsafe fn set_blending_enabled(&self, enabled: bool) {
    msg_send![self, setBlendingEnabled:enabled]
  }
}
