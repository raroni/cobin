use crate::{
  metal,
  PrivateMarker,
  obj::Object
};

pub struct CAMetalDrawable(PrivateMarker);

impl Object for CAMetalDrawable {}
unsafe impl objc::Message for CAMetalDrawable {}

impl CAMetalDrawable {
  pub unsafe fn texture(&self) -> *mut metal::MTLTexture {
    msg_send![self, texture]
  }
}
