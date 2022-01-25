use crate::metal::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLRegion {
  pub origin: MTLOrigin,
  pub size: MTLSize,
}

impl MTLRegion {
  pub fn new(origin: MTLOrigin, size: MTLSize) -> Self {
    Self { origin, size }
  }
}
