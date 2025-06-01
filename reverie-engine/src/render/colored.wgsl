const LOC_VERTEX: u32 = 0;
const LOC_COLOR: u32 = 1;
const GROUP_TRANSFORM: u32 = 0;
const BINDING_TRANSFORM: u32 = 0;

struct VertexInput {
  @location(LOC_VERTEX) position: vec3<f32>,
  @location(LOC_COLOR) color: vec4<f32>
}

struct VertexOutput {
  @location(0) color: vec4<f32>,
  @builtin(position) position: vec4<f32>
}

@group(GROUP_TRANSFORM)
@binding(BINDING_TRANSFORM)
var<uniform> transform: mat4x4<f32>;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
  var out: VertexOutput;
  out.color = in.color;
  out.position = transform * vec4<f32>(in.position, 1.0);
  return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  return in.color;
}
