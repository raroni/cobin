use crate::{
  PrivateMarker,
  core_foundation::*
};

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

#[repr(C)]
pub struct CGColorSpace(PrivateMarker);

pub type CGColorSpaceRef = *const CGColorSpace;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    pub static kCGColorSpaceSRGB: CFStringRef;
    pub static kCGColorSpaceAdobeRGB1998: CFStringRef;
    pub static kCGColorSpaceGenericGray: CFStringRef;
    pub static kCGColorSpaceGenericRGB: CFStringRef;
    pub static kCGColorSpaceGenericCMYK: CFStringRef;
    pub static kCGColorSpaceGenericRGBLinear: CFStringRef;
    pub static kCGColorSpaceGenericGrayGamma2_2: CFStringRef;

    pub fn CGColorSpaceCreateDeviceRGB() -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateDeviceGray() -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateWithName(name: CFStringRef) -> CGColorSpaceRef;
    pub fn CGColorSpaceRelease(cs: CGColorSpaceRef);
}
