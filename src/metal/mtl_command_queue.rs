use crate::{
  obj::Object,
  foundation::NSString,
  PrivateMarker,
  metal::*
};

pub struct MTLCommandQueue(PrivateMarker);

impl Object for MTLCommandQueue {}
unsafe impl objc::Message for MTLCommandQueue {}

impl MTLCommandQueue {
  pub unsafe fn command_buffer(&self) -> *mut MTLCommandBuffer {
    msg_send![self, commandBuffer]
  }

  pub unsafe fn set_label(&self, label: *mut NSString) {
    msg_send![self, setLabel:label]
  }
}
