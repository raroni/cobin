use crate::{
  obj::Object,
  foundation::NSPoint,
  app_kit::*,
  PrivateMarker,
  runtime::NSObjectBase
};

pub struct NSEvent(PrivateMarker);

impl Object for NSEvent {}
unsafe impl objc::Message for NSEvent {}

impl NSObjectBase for NSEvent {
  fn class_name() -> &'static str { "NSEvent" }
}

impl NSEvent {
  pub unsafe fn event_type(&self) -> NSEventType {
    msg_send![self, type]
  }

  pub unsafe fn key_code(&self) -> cty::c_ushort {
    msg_send![self, keyCode]
  }

  pub unsafe fn location_in_window(&self) -> NSPoint {
    msg_send![self, locationInWindow]
  }

  pub unsafe fn window(&self) -> *mut NSWindow {
    msg_send![self, window]
  }
}
