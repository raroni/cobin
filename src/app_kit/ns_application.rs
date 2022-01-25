use crate::{
  app_kit::*,
  foundation::{NSDate, NSRunLoopMode},
  obj::Object,
  PrivateMarker,
  runtime::NSObjectBase,
  util
};

pub struct NSApplication(PrivateMarker);

impl Object for NSApplication {}
unsafe impl objc::Message for NSApplication {}

impl NSObjectBase for NSApplication {
  fn class_name() -> &'static str { "NSApplication" }
}

#[allow(dead_code)]
impl NSApplication {
  pub unsafe fn shared_application() -> *mut Self {
    msg_send![Self::get_class(), sharedApplication]
  }

  pub unsafe fn finish_launching(&self) {
    msg_send![self, finishLaunching]
  }

  pub unsafe fn activate_ignoring_other_apps(&self, ignore: bool) {
    msg_send![self, activateIgnoringOtherApps:(ignore as objc::runtime::BOOL)]
  }

  pub unsafe fn set_main_menu(&self, menu: *mut NSMenu) {
    msg_send![self, setMainMenu:menu]
  }

  pub unsafe fn send_event(&self, event: *mut NSEvent) {
    msg_send![self, sendEvent:event]
  }

  pub unsafe fn set_activation_policy(&self, policy: NSApplicationActivationPolicy) -> bool {
    let result = msg_send![self, setActivationPolicy:policy];
    util::objc_bool_to_rust_bool(result)
  }

  pub unsafe fn next_event_matching_mask_until_date_in_mode_dequeue(&self, mask: NSEventMask, expiration: *mut NSDate, mode: *mut NSRunLoopMode, deq_flag: bool) -> *mut NSEvent {
    msg_send![self, nextEventMatchingMask:mask untilDate:expiration inMode:mode dequeue:(deq_flag as objc::runtime::BOOL)]
  }
}
