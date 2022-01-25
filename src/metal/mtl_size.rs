use crate::runtime::NSUInteger;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLSize {
  pub width: NSUInteger,
  pub height: NSUInteger,
  pub depth: NSUInteger,
}

impl MTLSize {
  pub fn new(width: NSUInteger, height: NSUInteger, depth: NSUInteger) -> Self {
    Self { width, height, depth }
  }
}
