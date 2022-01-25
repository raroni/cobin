use crate::{
  obj::Object,
  PrivateMarker,
  foundation::NSArray,
  runtime::NSObjectBase,
  metal::*
};

pub struct MTLPrimitiveAccelerationStructureDescriptor(PrivateMarker);

impl Object for MTLPrimitiveAccelerationStructureDescriptor {}
unsafe impl objc::Message for MTLPrimitiveAccelerationStructureDescriptor {}

impl NSObjectBase for MTLPrimitiveAccelerationStructureDescriptor {
  fn class_name() -> &'static str { "MTLPrimitiveAccelerationStructureDescriptor" }
}

impl MTLPrimitiveAccelerationStructureDescriptor {
  pub unsafe fn descriptor() -> *mut Self {
    let cls = Self::get_class();
    msg_send![cls, descriptor]
  }

  pub unsafe fn geometry_descriptors(&self) -> *mut NSArray<*mut MTLAccelerationStructureGeometryDescriptor> {
    msg_send![self, geometryDescriptors]
  }

  pub unsafe fn set_geometry_descriptors(&mut self, descriptors: *mut NSArray<*mut MTLAccelerationStructureGeometryDescriptor>) {
    msg_send![self, setGeometryDescriptors:descriptors]
  }
}

impl MTLAccelerationStructureDescriptorBase for MTLPrimitiveAccelerationStructureDescriptor { }
