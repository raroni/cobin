use crate::{
  obj::Object,
  PrivateMarker,
  runtime::NSObjectBase,
  metal::*
};

pub struct MTLAccelerationStructureTriangleGeometryDescriptor(PrivateMarker);

impl Object for MTLAccelerationStructureTriangleGeometryDescriptor {}
unsafe impl objc::Message for MTLAccelerationStructureTriangleGeometryDescriptor {}

impl NSObjectBase for MTLAccelerationStructureTriangleGeometryDescriptor {
  fn class_name() -> &'static str { "MTLAccelerationStructureTriangleGeometryDescriptor" }
}

impl MTLAccelerationStructureGeometryDescriptorBase for MTLAccelerationStructureTriangleGeometryDescriptor {}

impl MTLAccelerationStructureTriangleGeometryDescriptor {
  pub unsafe fn descriptor() -> *mut Self {
    let cls = Self::get_class();
    msg_send![cls, descriptor]
  }

  pub unsafe fn set_vertex_buffer(&mut self, buffer: *mut MTLBuffer) {
    msg_send![self, setVertexBuffer:buffer]
  }

  pub unsafe fn set_vertex_buffer_offset(&mut self, offset: NSUInteger) {
    msg_send![self, setVertexBufferOffset:offset]
  }

  pub unsafe fn set_triangle_count(&mut self, count: NSUInteger) {
    msg_send![self, setTriangleCount:count]
  }

  pub unsafe fn set_index_buffer(&mut self, buffer: *mut MTLBuffer) {
    msg_send![self, setIndexBuffer:buffer]
  }

  pub unsafe fn set_index_buffer_offset(&mut self, offset: NSUInteger) {
    msg_send![self, setIndexBufferOffset:offset]
  }

  pub unsafe fn set_index_type(&mut self, type_: MTLIndexType) {
    msg_send![self, setIndexType:type_]
  }
}
