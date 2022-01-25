use crate::{
  obj::Object,
  runtime::NSUInteger,
  PrivateMarker,
  metal::*,
  runtime::NSObjectBase
};

pub struct MTLRenderPipelineColorAttachmentDescriptorArray(PrivateMarker);

impl Object for MTLRenderPipelineColorAttachmentDescriptorArray {}
unsafe impl objc::Message for MTLRenderPipelineColorAttachmentDescriptorArray {}

impl NSObjectBase for MTLRenderPipelineColorAttachmentDescriptorArray {
  fn class_name() -> &'static str { "MTLRenderPipelineColorAttachmentDescriptorArray" }
}

impl MTLRenderPipelineColorAttachmentDescriptorArray {
  pub unsafe fn object_at_indexed_subscript(&self, index: NSUInteger) -> *mut MTLRenderPipelineColorAttachmentDescriptor {
    msg_send![self, objectAtIndexedSubscript:index]
  }
}
