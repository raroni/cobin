use crate::{
  obj::Object,
  PrivateMarker,
  runtime::NSObjectBase,
  foundation::NSArray,
  metal::*
};

pub struct MTLInstanceAccelerationStructureDescriptor(PrivateMarker);

impl Object for MTLInstanceAccelerationStructureDescriptor {}
unsafe impl objc::Message for MTLInstanceAccelerationStructureDescriptor {}

impl NSObjectBase for MTLInstanceAccelerationStructureDescriptor {
  fn class_name() -> &'static str { "MTLInstanceAccelerationStructureDescriptor" }
}

impl MTLInstanceAccelerationStructureDescriptor {
  pub unsafe fn descriptor() -> *mut Self {
    let cls = Self::get_class();
    msg_send![cls, descriptor]
  }

  pub unsafe fn set_instanced_acceleration_structures(&mut self, structures: *mut NSArray<*mut MTLAccelerationStructure>) {
    msg_send![self, setInstancedAccelerationStructures:structures]
  }

  pub unsafe fn set_instance_descriptor_buffer(&mut self, buffer: *mut MTLBuffer) {
    msg_send![self, setInstanceDescriptorBuffer:buffer]
  }

  pub unsafe fn set_instance_count(&mut self, count: NSUInteger) {
    msg_send![self, setInstanceCount:count]
  }

  pub unsafe fn set_instance_descriptor_buffer_offset(&mut self, offset: NSUInteger) {
    msg_send![self, setInstanceDescriptorBufferOffset:offset]
  }

  pub unsafe fn set_instance_descriptor_stride(&mut self, stride: NSUInteger) {
    msg_send![self, setInstanceDescriptorStride:stride]
  }
}

impl MTLAccelerationStructureDescriptorBase for MTLInstanceAccelerationStructureDescriptor { }
