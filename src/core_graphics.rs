pub type CGFloat = f64;

#[repr(C)]
pub struct CGPoint {
  pub x: CGFloat,
  pub y: CGFloat
}

#[repr(C)]
pub struct CGSize {
  pub width: CGFloat,
  pub height: CGFloat
}

impl CGSize {
  pub fn new(width: f64, height: f64) -> Self {
    Self {
      width: width as CGFloat,
      height: height as CGFloat
    }
  }
}

#[repr(C)]
pub struct CGRect {
  pub origin: CGPoint,
  pub size: CGSize
}
