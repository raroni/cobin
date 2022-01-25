use crate::{
  Selector,
  Strong,
  runtime::NSObjectBase,
  foundation::{NSString, NSStringEncoding, NSDefaultRunLoopMode, NSRunLoopMode},
  metal::{MTLDevice, MTLCreateSystemDefaultDevice}
};

pub fn selector(name: &str) -> Selector {
  objc::runtime::Sel::register(name)
}

pub unsafe fn string(string: &str) -> Strong<NSString> {
  let mut ptr = NSString::alloc();
  ptr.init(ptr.init_with_bytes_length_encoding(string.as_ptr(), string.len(), NSStringEncoding::UTF8));
  ptr
}

pub unsafe fn ns_default_run_loop_mode() -> *mut NSRunLoopMode {
  &mut *NSDefaultRunLoopMode
}

pub unsafe fn null_selector() -> Selector {
  objc::runtime::Sel::from_ptr(std::ptr::null_mut())
}

// TODO: I guess this ideally should use From<> and Into<>. However, I ran into
// an compiler issue when I tried that and decided to postpone for now.
pub fn objc_bool_to_rust_bool(bool: objc::runtime::BOOL) -> bool {
  bool == objc::runtime::YES
}

pub unsafe fn mtl_create_system_default_device() -> *mut MTLDevice {
  &mut *MTLCreateSystemDefaultDevice()
}
