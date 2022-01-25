use crate::{
  obj::Object,
  runtime::{
    NSObjectBase,
    NSUInteger
  },
  PrivateMarker,
  metal::*
};

pub struct MTLRenderPassColorAttachmentDescriptorArray(PrivateMarker);

impl Object for MTLRenderPassColorAttachmentDescriptorArray {}
unsafe impl objc::Message for MTLRenderPassColorAttachmentDescriptorArray {}

impl NSObjectBase for MTLRenderPassColorAttachmentDescriptorArray {
  fn class_name() -> &'static str { "MTLRenderPassColorAttachmentDescriptorArray" }
}

impl MTLRenderPassColorAttachmentDescriptorArray {
  pub unsafe fn object_at_indexed_subscript(&self, index: NSUInteger) -> *mut MTLRenderPassColorAttachmentDescriptor {
    msg_send![self, objectAtIndexedSubscript:index]
  }
}
