use crate::{
  obj::Object,
  foundation::*,
  runtime::NSObjectBase,
  PrivateMarker
};

pub struct NSBundle(PrivateMarker);

impl Object for NSBundle {}
unsafe impl objc::Message for NSBundle {}

impl NSObjectBase for NSBundle {
  fn class_name() -> &'static str { "NSBundle" }
}

impl NSBundle {
  pub unsafe fn main_bundle() -> *mut NSBundle {
    msg_send![Self::get_class(), mainBundle]
  }

  pub unsafe fn resource_path(&self) -> *mut NSString {
    msg_send![self, resourcePath]
  }
}
