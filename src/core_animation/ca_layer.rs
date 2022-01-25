use crate::{
  obj::Object,
  PrivateMarker,
  runtime::NSObjectBase
};

pub struct CALayer(PrivateMarker);

impl Object for CALayer {}
unsafe impl objc::Message for CALayer {}

impl NSObjectBase for CALayer {
  fn class_name() -> &'static str {
    "CALayer"
  }
}
