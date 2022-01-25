mod ns_object;

pub type NSUInteger = usize;
pub type NSInteger = isize;
pub const NIL: *mut NSObject = 0 as *mut NSObject;

pub use ns_object::{NSObject, NSObjectBase, NSObjectProtocol};
