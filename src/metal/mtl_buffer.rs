use crate::{
  obj::Object,
  PrivateMarker
};

pub struct MTLBuffer(PrivateMarker);

impl Object for MTLBuffer {}
unsafe impl objc::Message for MTLBuffer {}

impl MTLBuffer {
  pub unsafe fn contents(&self) -> *const std::ffi::c_void {
    msg_send![self, contents]
  }
}
