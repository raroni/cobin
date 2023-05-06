use crate::{
  runtime::NSUInteger,
  foundation::NSRange,
  obj::Object,
  PrivateMarker,
  metal::*
};

pub struct MTLBlitCommandEncoder(PrivateMarker);

impl Object for MTLBlitCommandEncoder {}
unsafe impl objc::Message for MTLBlitCommandEncoder {}

impl MTLCommandEncoderProtocol for MTLBlitCommandEncoder {}

impl MTLBlitCommandEncoder {
  pub unsafe fn copy_from_buffer_source_offset_to_buffer_destination_offset_size(
    &self,
    source_buffer: *mut MTLBuffer,
    source_offset: NSUInteger,
    destination_buffer: *mut MTLBuffer,
    destination_offset: NSUInteger,
    size: NSUInteger
  ) {
    msg_send![self, copyFromBuffer:source_buffer sourceOffset:source_offset toBuffer:destination_buffer destinationOffset:destination_offset size:size]
  }

  pub unsafe fn fill_buffer_range_value(
    &self,
    buffer: *mut MTLBuffer,
    range: NSRange,
    value: u8
  ) {
    msg_send![self, fillBuffer:buffer range:range value:value]
  }

  pub unsafe fn copy_from_texture_to_texture(&self, source: *mut MTLTexture, destination: *mut MTLTexture) {
    msg_send![self, copyFromTexture:source toTexture:destination]
  }
}
