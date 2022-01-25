use crate::runtime::NSUInteger;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLOrigin {
  pub x: NSUInteger,
  pub y: NSUInteger,
  pub z: NSUInteger,
}

impl MTLOrigin {
  pub fn new(x: NSUInteger, y: NSUInteger, z: NSUInteger) -> Self {
    Self { x, y, z }
  }
}
