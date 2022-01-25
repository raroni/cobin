use crate::{
  runtime::NSObjectBase,
  PrivateMarker,
  obj::Object
};

#[repr(C)]
pub struct NSRunLoopMode(PrivateMarker);

impl Object for NSRunLoopMode {}
unsafe impl objc::Message for NSRunLoopMode {}

impl NSObjectBase for NSRunLoopMode {
  fn class_name() -> &'static str { "NSRunLoopMode" }
}
