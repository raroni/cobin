use crate::{
  PrivateMarker,
  obj::Object,
  runtime::NSUInteger
};

pub struct MTLComputePipelineState(PrivateMarker);

impl Object for MTLComputePipelineState {}
unsafe impl objc::Message for MTLComputePipelineState {}

impl MTLComputePipelineState {
  pub unsafe fn thread_execution_width(&self) -> NSUInteger {
    msg_send![self, threadExecutionWidth]
  }

  pub unsafe fn max_total_threads_per_threadgroup(&self) -> NSUInteger {
    msg_send![self, maxTotalThreadsPerThreadgroup]
  }
}
