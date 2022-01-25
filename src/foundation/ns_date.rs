use crate::{
  obj::Object,
  runtime::NSObjectBase,
  PrivateMarker
};

pub struct NSDate(PrivateMarker);

impl Object for NSDate {}
unsafe impl objc::Message for NSDate {}

impl NSObjectBase for NSDate {
  fn class_name() -> &'static str { "NSDate" }
}

impl NSDate {
  pub unsafe fn distant_past() -> *mut Self {
    msg_send![Self::get_class(), distantPast]
  }
}
