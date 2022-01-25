use crate::{
  obj::Object,
  PrivateMarker,
  metal::*
};

pub struct MTLAccelerationStructure(PrivateMarker);

impl Object for MTLAccelerationStructure {}
unsafe impl objc::Message for MTLAccelerationStructure {}

impl MTLResourceProtocol for MTLAccelerationStructure {}

impl MTLAccelerationStructure {
}
