mod ns_application;
mod ns_menu;
mod ns_menu_item;
mod ns_event;
mod ns_window;
mod ns_view;

#[link(name = "AppKit", kind = "framework")]
extern {}

#[repr(isize)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSApplicationActivationPolicy {
  Regular = 0,
  Accessory = 1,
  Prohibited = 2,
  ERROR = -1
}

#[repr(u64)]
#[derive(Clone, Copy)]
pub enum NSEventMask {
  Any = std::u64::MAX
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(usize)]
pub enum NSEventType {
  LeftMouseDown         = 1,
  LeftMouseUp           = 2,
  RightMouseDown        = 3,
  RightMouseUp          = 4,
  MouseMoved            = 5,
  LeftMouseDragged      = 6,
  RightMouseDragged     = 7,
  MouseEntered          = 8,
  MouseExited           = 9,
  KeyDown               = 10,
  KeyUp                 = 11,
  FlagsChanged          = 12,
  AppKitDefined         = 13,
  SystemDefined         = 14,
  ApplicationDefined    = 15,
  Periodic              = 16,
  CursorUpdate          = 17,
  ScrollWheel           = 22,
  TabletPoint           = 23,
  TabletProximity       = 24,
  OtherMouseDown        = 25,
  OtherMouseUp          = 26,
  OtherMouseDragged     = 27,
  Gesture               = 29,
  Magnify               = 30,
  Swipe                 = 31,
  Rotate                = 18,
  BeginGesture          = 19,
  EndGesture            = 20,
  Pressure              = 34,
}

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum NSWindowStyleMask {
  Titled = 1
}

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum NSBackingStoreType {
  Buffered = 2
}

pub use ns_application::NSApplication;
pub use ns_menu::NSMenu;
pub use ns_menu_item::NSMenuItem;
pub use ns_event::NSEvent;
pub use ns_window::NSWindow;
pub use ns_view::NSView;
