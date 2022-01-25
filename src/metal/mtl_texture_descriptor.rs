use crate::{
  obj::Object,
  PrivateMarker,
  runtime::{
    NSObjectBase,
    NSUInteger
  },
  metal::*
};

pub struct MTLTextureDescriptor(PrivateMarker);

impl Object for MTLTextureDescriptor {}
unsafe impl objc::Message for MTLTextureDescriptor {}

impl NSObjectBase for MTLTextureDescriptor {
  fn class_name() -> &'static str { "MTLTextureDescriptor" }
}

impl MTLTextureDescriptor {
  pub unsafe fn set_pixel_format(&self, format: MTLPixelFormat) {
    msg_send![self, setPixelFormat:format]
  }

  pub unsafe fn set_texture_type(&self, typ: MTLTextureType) {
    msg_send![self, setTextureType: typ]
  }

  pub unsafe fn set_width(&self, width: NSUInteger) {
    msg_send![self, setWidth:width]
  }

  pub unsafe fn set_height(&self, height: NSUInteger) {
    msg_send![self, setHeight:height]
  }

  pub unsafe fn set_depth(&self, depth: NSUInteger) {
    msg_send![self, setDepth:depth]
  }

  pub unsafe fn set_usage(&self, usage: MTLTextureUsage) {
    msg_send![self, setUsage:usage]
  }

  pub unsafe fn set_storage_mode(&self, mode: MTLStorageMode) {
    msg_send![self, setStorageMode:mode]
  }
}
