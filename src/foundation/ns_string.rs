use crate::{
  runtime::{NSObjectBase, NSUInteger},
  foundation::*,
  PrivateMarker,
  obj::Object
};

pub struct NSString(PrivateMarker);

impl Object for NSString {}
unsafe impl objc::Message for NSString {}

impl NSObjectBase for NSString {
  fn class_name() -> &'static str { "NSString" }
}

impl NSString {
  pub unsafe fn init_with_bytes_length_encoding(&self, bytes: *const u8, length: NSUInteger, encoding: NSStringEncoding) -> *mut Self {
    msg_send![self, initWithBytes:bytes length:length encoding:encoding]
  }

  pub unsafe fn utf8_string(&self) -> *const std::os::raw::c_char {
    msg_send![self, UTF8String]
  }
}
