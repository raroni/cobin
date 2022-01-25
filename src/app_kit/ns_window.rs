use crate::{
  Strong,
  runtime::{NSObject, NSObjectBase},
  foundation::{NSRect, NSString},
  app_kit::*,
  PrivateMarker,
  obj::Object
};

pub struct NSWindow(PrivateMarker);

impl Object for NSWindow {}
unsafe impl objc::Message for NSWindow {}

impl NSObjectBase for NSWindow {
  fn class_name() -> &'static str { "NSWindow" }
}

impl NSWindow {
  pub unsafe fn new_with_content_rect_style_mask_backing_defer(
    content_rect: NSRect,
    style_mask: NSWindowStyleMask,
    backing: NSBackingStoreType,
    defer: bool // TODO: Make this objc::runtime::BOOL.
  ) -> Strong<Self> {
    let mut window = Self::alloc();
    let style_mask_integer = style_mask as u64;
    let backing_integer = backing as u64;
    let defer_native: objc::runtime::BOOL = defer.into();
    window.init(msg_send![window, initWithContentRect:content_rect styleMask:style_mask_integer backing:backing_integer defer:defer_native]);
    window
  }

  pub unsafe fn center(&self) {
    msg_send![self, center]
  }

  pub unsafe fn set_title(&self, title: *mut NSString) {
    msg_send![self, setTitle:title]
  }

  pub unsafe fn set_accepts_mouse_moved_events(&self, val: bool) {
    let val_native: objc::runtime::BOOL = val.into();
    msg_send![self, setAcceptsMouseMovedEvents:val_native]
  }

  pub unsafe fn make_key_and_order_front(&self, sender: *mut NSObject) {
    msg_send![self, makeKeyAndOrderFront:sender]
  }

  pub unsafe fn content_view(&self) -> *mut NSView {
    msg_send![self, contentView]
  }
}
