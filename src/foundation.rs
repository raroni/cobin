mod ns_string;
mod ns_date;
mod ns_array;
mod ns_mutable_array;
mod ns_error;
mod ns_run_loop_mode;
mod ns_bundle;

use crate::{
  core_graphics::{
    CGRect,
    CGPoint
  },
  runtime::NSUInteger
};

#[link(name = "Foundation", kind = "framework")]
extern {
  pub static NSDefaultRunLoopMode: *mut NSRunLoopMode;
}

pub type NSRect = CGRect;
pub type NSPoint = CGPoint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSRange {
  pub location: NSUInteger,
  pub length: NSUInteger
}

impl NSRange {
  pub fn new(location: NSUInteger, length: NSUInteger) -> Self {
    Self {
      location, length
    }
  }
}

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum NSStringEncoding {
  // ASCII = 1,
  // NEXTSTEP = 2,
  // JapaneseEUC = 3,
  UTF8 = 4
}

pub use ns_string::NSString;
pub use ns_date::NSDate;
pub use ns_error::NSError;
pub use ns_run_loop_mode::NSRunLoopMode;
pub use ns_bundle::NSBundle;
pub use ns_array::{NSArray, NSArrayBase};
pub use ns_mutable_array::NSMutableArray;
