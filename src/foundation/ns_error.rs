use crate::{
  PrivateMarker,
  obj::Object,
  runtime::NSObjectBase
};

pub struct NSError(PrivateMarker);

impl Object for NSError {}
unsafe impl objc::Message for NSError {}

impl NSObjectBase for NSError {
  fn class_name() -> &'static str { "NSError" }
}
