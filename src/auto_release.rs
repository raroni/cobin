// Slightly modified version of the objc crate's AutoReleaseHelper.

use std::os::raw::c_void;
use objc::runtime::{objc_autoreleasePoolPush, objc_autoreleasePoolPop};

pub struct AutoReleaseContext {
  context: *mut c_void,
}

impl AutoReleaseContext {
  pub unsafe fn new() -> Self {
    Self { context: objc_autoreleasePoolPush() }
  }
}

impl Drop for AutoReleaseContext {
  fn drop(&mut self) {
    unsafe { objc_autoreleasePoolPop(self.context) }
  }
}
