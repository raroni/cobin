#include <metal_stdlib>

using namespace metal;

struct RasterizerData {
  float4 position [[position]];
  float4 color;
};

vertex RasterizerData vs(
  uint vertex_idx [[vertex_id]],
  constant float2 *positions [[buffer(0)]],
  constant float2 &displacement [[buffer(1)]]
) {
  RasterizerData out;
  float2 position = positions[vertex_idx];
  out.position = float4(displacement + position, 0.0f, 1.0f);
  out.color = float4(1.0f, 0.0f, 0.0f, 0.0f);
  return out;
}

fragment float4 ps(RasterizerData in [[stage_in]]) {
  return in.color;
}
