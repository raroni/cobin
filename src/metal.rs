mod mtl_device;
mod mtl_command_queue;
mod mtl_command_buffer;
mod mtl_buffer;
mod mtl_compile_options;
mod mtl_library;
mod mtl_render_pipeline_descriptor;
mod mtl_render_pipeline_state;
mod mtl_command_encoder;
mod mtl_resource;
mod mtl_function;
mod mtl_compute_pipeline_state;
mod mtl_compute_pipeline_descriptor;
mod mtl_compute_pipeline_reflection;
mod mtl_texture_descriptor;
mod mtl_texture;
mod mtl_depth_stencil_descriptor;
mod mtl_depth_stencil_state;
mod mtl_render_pass_descriptor;
mod mtl_render_command_encoder;
mod mtl_compute_command_encoder;
mod mtl_drawable;
mod mtl_render_pass_color_attachment_descriptor_array;
mod mtl_render_pass_attachment_descriptor;
mod mtl_render_pipeline_color_attachment_descriptor_array;
mod mtl_render_pipeline_color_attachment_descriptor;
mod mtl_render_pass_color_attachment_descriptor;
mod mtl_render_pass_depth_attachment_descriptor;
mod mtl_region;
mod mtl_origin;
mod mtl_size;
mod mtl_acceleration_structure_command_encoder;
mod mtl_acceleration_structure;
mod mtl_acceleration_structure_descriptor;
mod mtl_acceleration_structure_geometry_descriptor;
mod mtl_primitive_acceleration_structure_descriptor;
mod mtl_instance_acceleration_structure_descriptor;
mod mtl_acceleration_structure_triangle_geometry_descriptor;

use crate::runtime::NSUInteger;

#[repr(usize)]
#[derive(Clone, Copy, PartialEq)]
pub enum MTLReadWriteTextureTier {
  None = 0,
  _1 = 1,
  _2 = 2
}

#[link(name = "Metal", kind = "framework")]
extern "C" {
  pub fn MTLCreateSystemDefaultDevice() -> *mut MTLDevice;
}

#[repr(u64)]
#[derive(Copy, Clone)]
pub enum MTLLoadAction {
  DontCare = 0,
  Load = 1,
  Clear = 2
}

#[repr(u64)]
#[derive(Copy, Clone)]
pub enum MTLTextureType {
    _1D = 0,
    _1DArray = 1,
    _2D = 2,
    _2DArray = 3,
    _2DMultisample = 4,
    Cube = 5,
    CubeArray = 6,
    _3D = 7
}

#[repr(u64)]
#[derive(Copy, Clone)]
pub enum MTLWinding {
  Clockwise = 0,
  CounterClockwise = 1
}

#[repr(u64)]
#[derive(Copy, Clone)]
pub enum MTLCullMode {
  None = 0,
  Front = 1,
  Back = 2,
}

#[repr(u64)]
#[derive(Copy, Clone)]
pub enum MTLCompareFunction {
  Never = 0,
  Less = 1,
  Equal = 2,
  LessEqual = 3,
  Greater = 4,
  NotEqual = 5,
  GreaterEqual = 6,
  Always = 7
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLIndexType {
    UInt16 = 0,
    UInt32 = 1,
}

#[repr(u64)]
#[derive(Copy, Clone)]
pub enum MTLPrimitiveType {
  Point = 0,
  Line = 1,
  LineStrip = 2,
  Triangle = 3,
  TriangleStrip = 4
}

#[repr(u64)]
#[derive(Clone, Copy)]
pub enum MTLCPUCacheMode {
  DefaultCache = 0,
  WriteCombined = 1
}

#[repr(u64)]
#[derive(Clone, Copy)]
pub enum MTLStorageMode {
  Shared = 0,
  Managed = 1,
  Private = 2,
  Memoryless = 3
}

pub const MTL_RESOURCE_CPU_CACHE_MODE_SHIFT: NSUInteger = 0;
pub const MTL_RESOURCE_STORAGE_MODE_SHIFT: NSUInteger = 4;

bitflags! {
  pub struct MTLResourceOptions: NSUInteger {
    const CPU_CACHE_MODE_DEFAULT_CACHE  = (MTLCPUCacheMode::DefaultCache as NSUInteger) << MTL_RESOURCE_CPU_CACHE_MODE_SHIFT;
    const CPU_CACHE_MODE_WRITE_COMBINED = (MTLCPUCacheMode::WriteCombined as NSUInteger) << MTL_RESOURCE_CPU_CACHE_MODE_SHIFT;

    const STORAGE_MODE_SHARED  = (MTLStorageMode::Shared as NSUInteger) << MTL_RESOURCE_STORAGE_MODE_SHIFT;
    const STORAGE_MODE_MANAGED = (MTLStorageMode::Managed as NSUInteger) << MTL_RESOURCE_STORAGE_MODE_SHIFT;
    const STORAGE_MODE_PRIVATE = (MTLStorageMode::Private as NSUInteger) << MTL_RESOURCE_STORAGE_MODE_SHIFT;
    const STORAGE_MODE_MEMORYLESS = (MTLStorageMode::Memoryless as NSUInteger) << MTL_RESOURCE_STORAGE_MODE_SHIFT;
  }
}

bitflags! {
  pub struct MTLResourceUsage: NSUInteger {
    const READ   = 1 << 0;
    const WRITE  = 1 << 1;
    const SAMPLE = 1 << 2;
  }
}

bitflags! {
  pub struct MTLPipelineOption: NSUInteger {
    const NONE                       = 0;
    const ARGUMENT_INFO              = 1 << 0;
    const BUFFER_TYPE_INFO           = 1 << 1;
    const FAIL_ON_BINARY_ACHIVE_MISS = 1 << 2;
  }
}

bitflags! {
  pub struct MTLAccelerationStructureInstanceOptions: u32 {
    const NONE                                            = 0;
    const DISABLE_TRIANGLE_CULLING                        = 1 << 0;
    const TRIANGLE_FRONT_FACING_WINDING_COUNTER_CLOCKWISE = 1 << 1;
    const OPAQUE                                          = 1 << 2;
    const NON_OPAQUE                                      = 1 << 3;
  }
}

bitflags! {
  pub struct MTLTextureUsage: NSUInteger {
    const UNKNOWN           = 0x0000;
    const SHADER_READ       = 0x0001;
    const SHADER_WRITE      = 0x0002;
    const RENDER_TARGET     = 0x0004;
    const PIXEL_FORMAT_VIEW = 0x0010;
  }
}

#[repr(u64)]
#[derive(Copy, Clone, Debug)]
pub enum MTLStoreAction {
  DontCare = 0,
  Store = 1,
  MultisampleResolve = 2,
  StoreAndMultisampleResolve = 3,
  Unknown = 4,
  CustomSampleDepthStore = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLClearColor {
  pub red: f64,
  pub green: f64,
  pub blue: f64,
  pub alpha: f64
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLAccelerationStructureSizes {
  pub acceleration_structure_size: NSUInteger,
  pub build_scratch_buffer_size: NSUInteger,
  pub refit_scratch_buffer_size: NSUInteger
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLAccelerationStructureInstanceDescriptor {
  pub transformation_matrix: MTLPackedFloat4x3,
  pub options: MTLAccelerationStructureInstanceOptions,
  pub mask: u32,
  pub intersection_function_table_offset: u32,
  pub acceleration_structure_index: u32
}

#[repr(C, align(4))]
#[derive(Copy, Clone)]
pub struct MTLPackedFloat3 {
  pub elements: [f32; 3]
}

impl MTLPackedFloat3 {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self {
      elements: [x, y, z]
    }
  }
}

#[repr(C, align(4))]
#[derive(Copy, Clone)]
pub struct MTLPackedFloat4x3 {
  pub columns: [MTLPackedFloat3; 4]
}

impl MTLPackedFloat4x3 {
  pub fn new(
    col1: MTLPackedFloat3,
    col2: MTLPackedFloat3,
    col3: MTLPackedFloat3,
    col4: MTLPackedFloat3
  ) -> Self {
    Self {
      columns: [col1, col2, col3, col4]
    }
  }
}

impl MTLClearColor {
  pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
    Self { red, green, blue, alpha }
  }
}

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MTLPixelFormat {
  Invalid = 0,
  A8Unorm = 1,
  R8Unorm = 10,
  R8Unorm_sRGB = 11,
  R8Snorm = 12,
  R8Uint = 13,
  R8Sint = 14,
  R16Unorm = 20,
  R16Snorm = 22,
  R16Uint = 23,
  R16Sint = 24,
  R16Float = 25,
  RG8Unorm = 30,
  RG8Unorm_sRGB = 31,
  RG8Snorm = 32,
  RG8Uint = 33,
  RG8Sint = 34,
  B5G6R5Unorm = 40,
  A1BGR5Unorm = 41,
  ABGR4Unorm = 42,
  BGR5A1Unorm = 43,
  R32Uint = 53,
  R32Sint = 54,
  R32Float = 55,
  RG16Unorm = 60,
  RG16Snorm = 62,
  RG16Uint = 63,
  RG16Sint = 64,
  RG16Float = 65,
  RGBA8Unorm = 70,
  RGBA8Unorm_sRGB = 71,
  RGBA8Snorm = 72,
  RGBA8Uint = 73,
  RGBA8Sint = 74,
  BGRA8Unorm = 80,
  BGRA8Unorm_sRGB = 81,
  RGB10A2Unorm = 90,
  RGB10A2Uint = 91,
  RG11B10Float = 92,
  RGB9E5Float = 93,
  BGR10A2Unorm = 94,
  RG32Uint = 103,
  RG32Sint = 104,
  RG32Float = 105,
  RGBA16Unorm = 110,
  RGBA16Snorm = 112,
  RGBA16Uint = 113,
  RGBA16Sint = 114,
  RGBA16Float = 115,
  RGBA32Uint = 123,
  RGBA32Sint = 124,
  RGBA32Float = 125,
  BC1_RGBA = 130,
  BC1_RGBA_sRGB = 131,
  BC2_RGBA = 132,
  BC2_RGBA_sRGB = 133,
  BC3_RGBA = 134,
  BC3_RGBA_sRGB = 135,
  BC4_RUnorm = 140,
  BC4_RSnorm = 141,
  BC5_RGUnorm = 142,
  BC5_RGSnorm = 143,
  BC6H_RGBFloat = 150,
  BC6H_RGBUfloat = 151,
  BC7_RGBAUnorm = 152,
  BC7_RGBAUnorm_sRGB = 153,
  PVRTC_RGB_2BPP = 160,
  PVRTC_RGB_2BPP_sRGB = 161,
  PVRTC_RGB_4BPP = 162,
  PVRTC_RGB_4BPP_sRGB = 163,
  PVRTC_RGBA_2BPP = 164,
  PVRTC_RGBA_2BPP_sRGB = 165,
  PVRTC_RGBA_4BPP = 166,
  PVRTC_RGBA_4BPP_sRGB = 167,
  EAC_R11Unorm = 170,
  EAC_R11Snorm = 172,
  EAC_RG11Unorm = 174,
  EAC_RG11Snorm = 176,
  EAC_RGBA8 = 178,
  EAC_RGBA8_sRGB = 179,
  ETC2_RGB8 = 180,
  ETC2_RGB8_sRGB = 181,
  ETC2_RGB8A1 = 182,
  ETC2_RGB8A1_sRGB = 183,
  ASTC_4x4_sRGB = 186,
  ASTC_5x4_sRGB = 187,
  ASTC_5x5_sRGB = 188,
  ASTC_6x5_sRGB = 189,
  ASTC_6x6_sRGB = 190,
  ASTC_8x5_sRGB = 192,
  ASTC_8x6_sRGB = 193,
  ASTC_8x8_sRGB = 194,
  ASTC_10x5_sRGB = 195,
  ASTC_10x6_sRGB = 196,
  ASTC_10x8_sRGB = 197,
  ASTC_10x10_sRGB = 198,
  ASTC_12x10_sRGB = 199,
  ASTC_12x12_sRGB = 200,
  ASTC_4x4_LDR = 204,
  ASTC_5x4_LDR = 205,
  ASTC_5x5_LDR = 206,
  ASTC_6x5_LDR = 207,
  ASTC_6x6_LDR = 208,
  ASTC_8x5_LDR = 210,
  ASTC_8x6_LDR = 211,
  ASTC_8x8_LDR = 212,
  ASTC_10x5_LDR = 213,
  ASTC_10x6_LDR = 214,
  ASTC_10x8_LDR = 215,
  ASTC_10x10_LDR = 216,
  ASTC_12x10_LDR = 217,
  ASTC_12x12_LDR = 218,
  GBGR422 = 240,
  BGRG422 = 241,
  Depth16Unorm = 250,
  Depth32Float = 252,
  Stencil8 = 253,
  Depth24Unorm_Stencil8 = 255,
  Depth32Float_Stencil8 = 260,
  X32_Stencil8 = 261,
  X24_Stencil8 = 262,
  BGRA10_XR = 552,
  BGRA10_XR_SRGB = 553,
  BGR10_XR = 554,
  BGR10_XR_SRGB = 555,
}

#[repr(isize)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MTLGPUFamily {
  APPLE1 = 1001,
  APPLE2 = 1002,
  APPLE3 = 1003,
  APPLE4 = 1004,
  APPLE5 = 1005,
  APPLE6 = 1006,
  APPLE7 = 1007,
  MAC1 = 2001,
  MAC2 = 2002,
  COMMON1 = 3001,
  COMMON2 = 3002,
  COMMON3 = 3003,
  MAC_CATALYST1 = 4001,
  MAC_CATALYST2 = 4002
}

pub use mtl_device::MTLDevice;
pub use mtl_command_queue::MTLCommandQueue;
pub use mtl_buffer::MTLBuffer;
pub use mtl_compile_options::MTLCompileOptions;
pub use mtl_library::MTLLibrary;
pub use mtl_render_pipeline_descriptor::MTLRenderPipelineDescriptor;
pub use mtl_render_pipeline_state::MTLRenderPipelineState;
pub use mtl_function::MTLFunction;
pub use mtl_compute_pipeline_state::MTLComputePipelineState;
pub use mtl_compute_pipeline_descriptor::MTLComputePipelineDescriptor;
pub use mtl_compute_pipeline_reflection::MTLComputePipelineReflection;
pub use mtl_texture_descriptor::MTLTextureDescriptor;
pub use mtl_texture::MTLTexture;
pub use mtl_depth_stencil_descriptor::MTLDepthStencilDescriptor;
pub use mtl_depth_stencil_state::MTLDepthStencilState;
pub use mtl_command_buffer::MTLCommandBuffer;
pub use mtl_render_pass_descriptor::MTLRenderPassDescriptor;
pub use mtl_render_command_encoder::MTLRenderCommandEncoder;
pub use mtl_compute_command_encoder::MTLComputeCommandEncoder;
pub use mtl_drawable::MTLDrawable;
pub use mtl_render_pipeline_color_attachment_descriptor_array::MTLRenderPipelineColorAttachmentDescriptorArray;
pub use mtl_render_pipeline_color_attachment_descriptor::MTLRenderPipelineColorAttachmentDescriptor;
pub use mtl_render_pass_color_attachment_descriptor_array::MTLRenderPassColorAttachmentDescriptorArray;
pub use mtl_render_pass_color_attachment_descriptor::MTLRenderPassColorAttachmentDescriptor;
pub use mtl_render_pass_depth_attachment_descriptor::MTLRenderPassDepthAttachmentDescriptor;
pub use mtl_region::MTLRegion;
pub use mtl_origin::MTLOrigin;
pub use mtl_size::MTLSize;
pub use mtl_render_pass_attachment_descriptor::MTLRenderPassAttachmentDescriptorBase;
pub use mtl_command_encoder::MTLCommandEncoderProtocol;
pub use mtl_resource::MTLResourceProtocol;
pub use mtl_acceleration_structure::MTLAccelerationStructure;
pub use mtl_acceleration_structure_command_encoder::MTLAccelerationStructureCommandEncoder;
pub use mtl_acceleration_structure_descriptor::{MTLAccelerationStructureDescriptorBase, MTLAccelerationStructureDescriptor};
pub use mtl_primitive_acceleration_structure_descriptor::MTLPrimitiveAccelerationStructureDescriptor;
pub use mtl_instance_acceleration_structure_descriptor::MTLInstanceAccelerationStructureDescriptor;
pub use mtl_acceleration_structure_geometry_descriptor::{MTLAccelerationStructureGeometryDescriptorBase, MTLAccelerationStructureGeometryDescriptor};
pub use mtl_acceleration_structure_triangle_geometry_descriptor::MTLAccelerationStructureTriangleGeometryDescriptor;

pub type MTLAutoreleasedComputePipelineReflection = *mut MTLComputePipelineReflection;
