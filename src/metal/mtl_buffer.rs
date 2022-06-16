use crate::{
  obj::Object,
  PrivateMarker,
  foundation::NSRange,
  runtime::NSUInteger
};

pub struct MTLBuffer(PrivateMarker);

impl Object for MTLBuffer {}
unsafe impl objc::Message for MTLBuffer {}

impl MTLBuffer {
  pub unsafe fn contents(&self) -> *mut std::ffi::c_void {
    msg_send![self, contents]
  }

  pub unsafe fn length(&self) -> NSUInteger {
    msg_send![self, length]
  }

  pub unsafe fn did_modify_range(&self, range: NSRange) {
    msg_send![self, didModifyRange:range]
  }
}
