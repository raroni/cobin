use crate::{
  obj::Object,
  PrivateMarker,
  metal::*,
  foundation::NSString
};

pub struct MTLCommandBuffer(PrivateMarker);

impl Object for MTLCommandBuffer {}
unsafe impl objc::Message for MTLCommandBuffer {}

impl MTLCommandBuffer {
  pub unsafe fn render_command_encoder_with_descriptor(&self, descriptor: *mut MTLRenderPassDescriptor) -> *mut MTLRenderCommandEncoder {
    msg_send![self, renderCommandEncoderWithDescriptor:descriptor]
  }

  pub unsafe fn compute_command_encoder(&self) -> *mut MTLComputeCommandEncoder {
    msg_send![self, computeCommandEncoder]
  }

  pub unsafe fn acceleration_structure_command_encoder(&self) -> *mut MTLAccelerationStructureCommandEncoder {
    msg_send![self, accelerationStructureCommandEncoder]
  }

  pub unsafe fn commit(&self) {
    msg_send![self, commit]
  }

  pub unsafe fn status(&self) -> MTLCommandBufferStatus {
    msg_send![self, status]
  }

  pub unsafe fn wait_until_completed(&self) {
    msg_send![self, waitUntilCompleted]
  }

  pub unsafe fn add_completed_handler(&self, block: MTLCommandBufferHandler) {
    msg_send![self, addCompletedHandler:block]
  }

  pub unsafe fn set_label(&self, label: *mut NSString) {
    msg_send![self, setLabel:label]
  }

  pub unsafe fn label(&self) -> *mut NSString {
    msg_send![self, label]
  }

  pub unsafe fn present_drawable(&self, drawable: *mut MTLDrawable) {
    msg_send![self, presentDrawable:drawable]
  }
}
