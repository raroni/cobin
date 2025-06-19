use crate::{
  obj::Object,
  PrivateMarker,
  runtime::{NSObjectBase, NSUInteger}
};

pub trait MTLAccelerationStructureGeometryDescriptorBase: Object {
  unsafe fn set_intersection_function_table_offset(&mut self, offset: NSUInteger) {
    msg_send![self, setIntersectionFunctionTableOffset:offset]
  }

  unsafe fn set_opaque(&mut self, v: bool) {
    let opaque: objc::runtime::BOOL = v.into();
    msg_send![self, setOpaque:opaque]
  }
}

pub struct MTLAccelerationStructureGeometryDescriptor(PrivateMarker);

unsafe impl objc::Message for MTLAccelerationStructureGeometryDescriptor {}
impl Object for MTLAccelerationStructureGeometryDescriptor {}

impl NSObjectBase for MTLAccelerationStructureGeometryDescriptor {
  fn class_name() -> &'static str { "NSObject" }
}

impl MTLAccelerationStructureGeometryDescriptorBase for MTLAccelerationStructureGeometryDescriptor {
}
