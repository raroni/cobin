use crate::{
  obj::Object,
  runtime::NSObjectBase,
  PrivateMarker,
  app_kit::NSMenu
};

pub struct NSMenuItem(PrivateMarker);

impl Object for NSMenuItem {}
unsafe impl objc::Message for NSMenuItem {}

impl NSObjectBase for NSMenuItem {
  fn class_name() -> &'static str { "NSMenuItem" }
}

impl NSMenuItem {
  pub unsafe fn set_submenu(&self, menu: *mut NSMenu) {
    msg_send![self, setSubmenu:menu]
  }
}
