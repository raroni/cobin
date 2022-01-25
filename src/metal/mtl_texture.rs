use crate::{
  obj::Object,
  PrivateMarker,
  runtime::NSUInteger,
  metal::*
};

pub struct MTLTexture(PrivateMarker);

impl Object for MTLTexture {}
unsafe impl objc::Message for MTLTexture {}

impl MTLTexture {
  pub unsafe fn replace_region_mipmap_level_slice_with_bytes_bytes_per_row_bytes_per_image(
    &self,
    region: MTLRegion,
    mipmap_level: NSUInteger,
    slice: NSUInteger,
    bytes: *const std::ffi::c_void,
    bytes_per_row: NSUInteger,
    bytes_per_image: NSUInteger
  ) {
    msg_send![self, replaceRegion:region mipmapLevel:mipmap_level slice:slice withBytes:bytes bytesPerRow:bytes_per_row bytesPerImage:bytes_per_image]
  }

  pub unsafe fn width(&self) -> NSUInteger {
    msg_send![self, width]
  }

  pub unsafe fn height(&self) -> NSUInteger {
    msg_send![self, height]
  }

  pub unsafe fn replace_region_mipmap_level_with_bytes_bytes_per_row(
    &self,
    region: MTLRegion,
    mipmap_level: NSUInteger,
    bytes: *const std::ffi::c_void,
    bytes_per_row: NSUInteger,
  ) {
    msg_send![self, replaceRegion:region mipmapLevel:mipmap_level withBytes:bytes bytesPerRow:bytes_per_row]
  }

  pub unsafe fn get_bytes_bytes_per_row_from_region_mipmap_level(
    &self,
    bytes: *const std::ffi::c_void,
    bytes_per_row: NSUInteger,
    region: MTLRegion,
    level: NSUInteger
  ) {
    msg_send![self, getBytes:bytes bytesPerRow:bytes_per_row fromRegion:region mipmapLevel:level]
  }
}
