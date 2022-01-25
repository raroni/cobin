use crate::{
  obj::Object,
  runtime::NSUInteger,
  PrivateMarker,
  metal::*
};

pub struct MTLComputeCommandEncoder(PrivateMarker);

impl Object for MTLComputeCommandEncoder {}
unsafe impl objc::Message for MTLComputeCommandEncoder {}

impl MTLCommandEncoderProtocol for MTLComputeCommandEncoder {}

impl MTLComputeCommandEncoder {
  pub unsafe fn set_compute_pipeline_state(&self, pipeline_state: *mut MTLComputePipelineState) {
    msg_send![self, setComputePipelineState:pipeline_state]
  }

  pub unsafe fn set_buffer_offset_at_index(&self, buffer: *mut MTLBuffer, offset: NSUInteger, index: NSUInteger) {
    msg_send![self, setBuffer:buffer offset:offset atIndex:index]
  }

  pub unsafe fn dispatch_threads_threads_per_threadgroup(&self, threads_per_grid: MTLSize, threads_per_threadgroup: MTLSize) {
    msg_send![self, dispatchThreads:threads_per_grid threadsPerThreadgroup:threads_per_threadgroup]
  }

  pub unsafe fn dispatch_threadgroups_threads_per_threadgroup(&self, threadgroup_count: MTLSize, threads_per_threadgroup: MTLSize) {
    msg_send![self, dispatchThreadgroups:threadgroup_count threadsPerThreadgroup:threads_per_threadgroup]
  }

  pub unsafe fn set_bytes_length_at_index(&self, bytes: *const std::ffi::c_void, length: NSUInteger, index: NSUInteger) {
    msg_send![self, setBytes:bytes length:length atIndex:index]
  }

  pub unsafe fn set_texture_at_index(&self, texture: *mut MTLTexture, index: NSUInteger) {
    msg_send![self, setTexture:texture atIndex:index]
  }

  pub unsafe fn set_acceleration_structure_at_buffer_index(&self, structure: *mut MTLAccelerationStructure, index: NSUInteger) {
    msg_send![self, setAccelerationStructure:structure atBufferIndex:index]
  }

  pub unsafe fn use_resource_usage<T: MTLResourceProtocol>(&self, resource: *mut T, usage: MTLResourceUsage) {
    msg_send![self, useResource:resource usage:usage]
  }
}
