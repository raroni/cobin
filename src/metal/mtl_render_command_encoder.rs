use crate::{
  PrivateMarker,
  obj::Object,
  metal::*
};

pub struct MTLRenderCommandEncoder(PrivateMarker);

impl Object for MTLRenderCommandEncoder {}
unsafe impl objc::Message for MTLRenderCommandEncoder {}

impl MTLCommandEncoderProtocol for MTLRenderCommandEncoder {}

impl MTLRenderCommandEncoder {
  pub unsafe fn set_render_pipeline_state(&self, pipeline_state: *mut MTLRenderPipelineState) {
    msg_send![self, setRenderPipelineState:pipeline_state]
  }

  pub unsafe fn set_depth_stencil_state(&self, state: *mut MTLDepthStencilState) {
    msg_send![self, setDepthStencilState:state]
  }

  pub unsafe fn set_front_facing_winding(&self, winding: MTLWinding) {
    msg_send![self, setFrontFacingWinding:winding]
  }

  pub unsafe fn set_cull_mode(&self, mode: MTLCullMode) {
    msg_send![self, setCullMode:mode]
  }

  pub unsafe fn set_vertex_buffer_offset_at_index(&self, buffer: *mut MTLBuffer, offset: NSUInteger, index: NSUInteger) {
    msg_send![self, setVertexBuffer:buffer offset:offset atIndex:index]
  }

  pub unsafe fn set_vertex_bytes_offset_at_index(&self, bytes: *const std::ffi::c_void, length: NSUInteger, index: NSUInteger) {
    msg_send![self, setVertexBytes:bytes length:length atIndex:index]
  }

  pub unsafe fn set_fragment_buffer_offset_at_index(&self, buffer: *mut MTLBuffer, offset: NSUInteger, index: NSUInteger) {
    msg_send![self, setFragmentBuffer:buffer offset:offset atIndex:index]
  }

  pub unsafe fn set_fragment_bytes_offset_at_index(&self, bytes: *const std::ffi::c_void, length: NSUInteger, index: NSUInteger) {
    msg_send![self, setFragmentBytes:bytes length:length atIndex:index]
  }

  pub unsafe fn set_fragment_texture_at_index(&self, texture: *mut MTLTexture, index: NSUInteger) {
    msg_send![self, setFragmentTexture:texture atIndex:index]
  }

  pub unsafe fn set_fragment_acceleration_structure_at_buffer_index(&self, structure: *mut MTLAccelerationStructure, index: NSUInteger) {
    msg_send![self, setFragmentAccelerationStructure:structure atBufferIndex:index]
  }

  pub unsafe fn set_vertex_acceleration_structure_at_buffer_index(&self, structure: *mut MTLAccelerationStructure, index: NSUInteger) {
    msg_send![self, setVertexAccelerationStructure:structure atBufferIndex:index]
  }

  pub unsafe fn use_resource_usage<T: MTLResourceProtocol>(&self, resource: *mut T, usage: MTLResourceUsage, stages: MTLRenderStages) {
    msg_send![self, useResource:resource usage:usage stages:stages]
  }

  pub unsafe fn draw_primitives_vertex_start_vertex_count(&self, primitive_type: MTLPrimitiveType, vertex_start: NSUInteger, vertex_count: NSUInteger) {
    msg_send![self, drawPrimitives:primitive_type vertexStart:vertex_start vertexCount:vertex_count]
  }

  pub unsafe fn draw_indexed_primitives_index_count_index_type_index_buffer_index_buffer_offset(
    &self,
    primitive_type: MTLPrimitiveType,
    index_count: NSUInteger,
    index_type: MTLIndexType,
    index_buffer: *mut MTLBuffer,
    index_buffer_offset: NSUInteger
  ) {
    msg_send![self, drawIndexedPrimitives:primitive_type
                    indexCount:index_count
                    indexType:index_type
                    indexBuffer:index_buffer
                    indexBufferOffset:index_buffer_offset]
  }
}
