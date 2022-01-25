use crate::{
  Selector,
  obj::Object,
  app_kit::*,
  PrivateMarker,
  foundation::NSString,
  runtime::NSObjectBase
};

pub struct NSMenu(PrivateMarker);

impl Object for NSMenu {}
unsafe impl objc::Message for NSMenu {}

impl NSObjectBase for NSMenu {
  fn class_name() -> &'static str { "NSMenu" }
}

impl NSMenu {
  pub unsafe fn add_item_with_title_action_key_equivalent(&self, title: *mut NSString, selector: Selector, key: *mut NSString) {
    msg_send![self, addItemWithTitle:title action:selector keyEquivalent:key]
  }

  pub unsafe fn add_item(&self, item: *mut NSMenuItem) {
    msg_send![self, addItem:item]
  }
}
