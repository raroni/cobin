use crate::{
  Strong,
  obj::Object,
  runtime::NSUInteger,
  PrivateMarker,
  foundation::{NSError, NSString},
  metal::*,
  util
};

#[repr(C)]
pub struct MTLDevice(PrivateMarker);

impl Object for MTLDevice {}
unsafe impl objc::Message for MTLDevice {}

impl MTLDevice {
  pub unsafe fn new_command_queue(&self) -> Strong<MTLCommandQueue> {
    let ptr = msg_send![self, newCommandQueue];
    Strong::new(ptr)
  }

  pub unsafe fn read_write_texture_support(&self) -> MTLReadWriteTextureTier {
    msg_send![self, readWriteTextureSupport]
  }

  pub unsafe fn supports_family(&self, family: MTLGPUFamily) -> bool {
    let result = msg_send![self, supportsFamily:family];
    util::objc_bool_to_rust_bool(result)
  }

  pub unsafe fn new_buffer_with_bytes_length_options(&self, bytes: *const std::ffi::c_void, length: NSUInteger, options: MTLResourceOptions) -> Strong<MTLBuffer> {
    let ptr = msg_send![self, newBufferWithBytes:bytes length:length options:options];
    Strong::new(ptr)
  }

  pub unsafe fn new_buffer_with_length_options(&self, length: NSUInteger, options: MTLResourceOptions) -> Strong<MTLBuffer> {
    let ptr = msg_send![self, newBufferWithLength:length options:options];
    Strong::new(ptr)
  }

  pub unsafe fn new_library_with_source_options_error(&self, source: *mut NSString, options: *mut MTLCompileOptions, error: *mut NSError) -> Strong<MTLLibrary> {
    let ptr = msg_send![self, newLibraryWithSource:source options:options error:&error];
    Strong::new(ptr)
  }

  pub unsafe fn new_library_with_file_error(&self, filepath: *mut NSString, error: *mut NSError) -> Strong<MTLLibrary> {
    let ptr = msg_send![self, newLibraryWithFile:filepath error:&error];
    Strong::new(ptr)
  }

  pub unsafe fn new_render_pipeline_state_with_descriptor_error(&self, descriptor: *mut MTLRenderPipelineDescriptor, error: *mut NSError) -> Strong<MTLRenderPipelineState> {
    let ptr = msg_send![self, newRenderPipelineStateWithDescriptor:descriptor error:&error];
    Strong::new(ptr)
  }

  pub unsafe fn new_compute_pipeline_state_with_function_error(&self, function: *mut MTLFunction, error: *mut NSError) -> Strong<MTLComputePipelineState> {
    let ptr = msg_send![self, newComputePipelineStateWithFunction:function error:&error];
    Strong::new(ptr)
  }

  pub unsafe fn new_compute_pipeline_state_with_descriptor_options_reflection_error(
    &self,
    descriptor: *mut MTLComputePipelineDescriptor,
    options: MTLPipelineOption,
    reflection: *mut MTLAutoreleasedComputePipelineReflection,
    error: *mut NSError
  ) -> Strong<MTLComputePipelineState> {
    let ptr = msg_send![self, newComputePipelineStateWithDescriptor:descriptor options:options reflection:reflection error:&error];
    Strong::new(ptr)
  }

  pub unsafe fn new_texture_with_descriptor(&self, descriptor: *mut MTLTextureDescriptor) -> Strong<MTLTexture> {
    let ptr = msg_send![self, newTextureWithDescriptor:descriptor];
    Strong::new(ptr)
  }

  pub unsafe fn new_depth_stencil_state_with_descriptor(&self, descriptor: *mut MTLDepthStencilDescriptor) -> Strong<MTLDepthStencilState> {
    let ptr = msg_send![self, newDepthStencilStateWithDescriptor:descriptor];
    Strong::new(ptr)
  }

  pub unsafe fn acceleration_structure_sizes_with_descriptor(&self, descriptor: *mut MTLAccelerationStructureDescriptor) -> MTLAccelerationStructureSizes {
    msg_send![self, accelerationStructureSizesWithDescriptor:descriptor]
  }

  pub unsafe fn new_acceleration_structure_with_size(&self, size: NSUInteger) -> Strong<MTLAccelerationStructure> {
    let ptr = msg_send![self, newAccelerationStructureWithSize:size];
    Strong::new(ptr)
  }
}
