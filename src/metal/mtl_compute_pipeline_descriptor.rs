use crate::{
  obj::Object,
  PrivateMarker,
  runtime::NSObjectBase,
  foundation::NSString,
  metal::*
};

pub struct MTLComputePipelineDescriptor(PrivateMarker);

impl Object for MTLComputePipelineDescriptor {}
unsafe impl objc::Message for MTLComputePipelineDescriptor {}

impl NSObjectBase for MTLComputePipelineDescriptor {
  fn class_name() -> &'static str { "MTLComputePipelineDescriptor" }
}

impl MTLComputePipelineDescriptor {
  pub unsafe fn set_label(&self, label: *mut NSString) {
    msg_send![self, setLabel:label]
  }

  pub unsafe fn set_compute_function(&self, func: *mut MTLFunction) {
    msg_send![self, setComputeFunction:func]
  }

  pub unsafe fn set_thread_group_size_is_multiple_of_thread_execution_width(&self, v: bool) {
    msg_send![self, setThreadGroupSizeIsMultipleOfThreadExecutionWidth:v]
  }

  pub unsafe fn set_max_total_threads_per_threadgroup(&self, v: NSUInteger) {
    msg_send![self, setMaxTotalThreadsPerThreadgroup:v]
  }
}
