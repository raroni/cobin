use crate::{
  obj::Object,
  PrivateMarker,
  metal::*
};

pub struct MTLAccelerationStructureCommandEncoder(PrivateMarker);

impl Object for MTLAccelerationStructureCommandEncoder {}
unsafe impl objc::Message for MTLAccelerationStructureCommandEncoder {}

impl MTLCommandEncoderProtocol for MTLAccelerationStructureCommandEncoder {}

impl MTLAccelerationStructureCommandEncoder {
  pub unsafe fn build_acceleration_structure_descriptor_scratch_buffer_scratch_buffer_offset(
    &mut self,
    structure: *mut MTLAccelerationStructure,
    descriptor: *mut MTLAccelerationStructureDescriptor,
    scratch_buffer: *mut MTLBuffer,
    offset: NSUInteger
  ) {
    msg_send![self, buildAccelerationStructure:structure descriptor:descriptor scratchBuffer:scratch_buffer scratchBufferOffset:offset]
  }
}
