use crate::{
  PrivateMarker,
  obj::Object
};

pub struct MTLDrawable(PrivateMarker);

impl Object for MTLDrawable {}
unsafe impl objc::Message for MTLDrawable {}

impl MTLDrawable {
  pub unsafe fn present(&self) {
    msg_send![self, present]
  }
}
