const LOC_VERTEX: u32 = 0;
const LOC_UV: u32 = 1;
const GROUP_TEXTURE: u32 = 0;
const BINDING_TEXTURE: u32 = 0;
const BINDING_SAMPLER: u32 = 1;
const GROUP_TRANSFORM: u32 = 1;
const BINDING_TRANSFORM: u32 = 0;

struct VertexInput {
  @location(LOC_VERTEX) position: vec3<f32>,
  @location(LOC_UV) uv: vec2<f32>
}

struct VertexOutput {
  @location(LOC_VERTEX) uv: vec2<f32>,
  @builtin(position) position: vec4<f32>
}

@group(GROUP_TEXTURE)
@binding(BINDING_TEXTURE)
var tex: texture_2d<f32>;

@group(GROUP_TEXTURE)
@binding(BINDING_SAMPLER)
var samp: sampler;

@group(GROUP_TRANSFORM)
@binding(BINDING_TRANSFORM)
var<uniform> transform: mat4x4<f32>;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
  var out: VertexOutput;
  out.uv = in.uv;
  out.position = transform * vec4<f32>(in.position, 1.0);
  return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  //return vec4<f32>(0.5, 0.5, 0.5, 1.0);
  return textureSample(tex, samp, in.uv);
}
