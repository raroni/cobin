use crate::{
  metal,
  obj::Object,
  core_graphics::*,
  core_animation::*,
  PrivateMarker,
  runtime::NSObjectBase
};

pub struct CAMetalLayer(PrivateMarker);

impl Object for CAMetalLayer {}
unsafe impl objc::Message for CAMetalLayer {}

impl NSObjectBase for CAMetalLayer {
  fn class_name() -> &'static str {
    "CAMetalLayer"
  }
}

impl CAMetalLayer {
  pub unsafe fn set_device(&self, device: *mut metal::MTLDevice) {
    msg_send![self, setDevice:device]
  }

  pub unsafe fn set_drawable_size(&self, size: CGSize) {
    msg_send![self, setDrawableSize: size]
  }

  pub unsafe fn set_pixel_format(&self, pixel_format: metal::MTLPixelFormat) {
    msg_send![self, setPixelFormat: pixel_format]
  }

  pub unsafe fn set_color_space(&self, space: CGColorSpaceRef) {
    msg_send![self, setColorspace:space]
  }

  pub unsafe fn set_presents_with_transaction(&self, transaction: bool) {
    let transaction_native: objc::runtime::BOOL = transaction.into();
    msg_send![self, setPresentsWithTransaction: transaction_native]
  }

  pub unsafe fn next_drawable(&self) -> *mut CAMetalDrawable {
    msg_send![self, nextDrawable]
  }

  pub unsafe fn framebuffer_only(&self) -> bool {
    msg_send![self, framebufferOnly]
  }

  pub unsafe fn set_framebuffer_only(&self, v: bool) -> bool {
    let native_bool: objc::runtime::BOOL = v.into();
    msg_send![self, setFramebufferOnly: native_bool]
  }
}
