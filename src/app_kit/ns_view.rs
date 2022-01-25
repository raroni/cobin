use crate::{
  runtime::NSObjectBase,
  core_animation::CALayer,
  PrivateMarker,
  obj::Object
};

pub struct NSView(PrivateMarker);

impl Object for NSView {}
unsafe impl objc::Message for NSView {}

impl NSObjectBase for NSView {
  fn class_name() -> &'static str {
    "NSView"
  }
}

impl NSView {
  pub unsafe fn set_wants_layer(&self, wants_layer: bool) {
    let wants_layer_native: objc::runtime::BOOL = wants_layer.into();
    msg_send![self, setWantsLayer:wants_layer_native]
  }

  pub unsafe fn set_layer(&self, layer: *mut CALayer) {
    msg_send![self, setLayer:layer]
  }
}
