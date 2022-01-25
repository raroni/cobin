use crate::{
  obj::Object,
  runtime::NSObjectBase,
  PrivateMarker,
  metal::*
};

pub struct MTLRenderPipelineDescriptor(PrivateMarker);

impl Object for MTLRenderPipelineDescriptor {}
unsafe impl objc::Message for MTLRenderPipelineDescriptor {}

impl NSObjectBase for MTLRenderPipelineDescriptor {
  fn class_name() -> &'static str { "MTLRenderPipelineDescriptor" }
}

impl MTLRenderPipelineDescriptor {
  pub unsafe fn set_fragment_function(&self, function: *mut MTLFunction) {
    msg_send![self, setFragmentFunction: function]
  }

  pub unsafe fn set_vertex_function(&self, function: *mut MTLFunction) {
    msg_send![self, setVertexFunction: function]
  }

  pub unsafe fn color_attachments(&self) -> *mut MTLRenderPipelineColorAttachmentDescriptorArray {
    msg_send![self, colorAttachments]
  }

  pub unsafe fn set_depth_attachment_pixel_format(&self, format: MTLPixelFormat) {
    msg_send![self, setDepthAttachmentPixelFormat:format]
  }
}
