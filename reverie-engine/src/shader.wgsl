struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) uv: vec2<f32>
}

struct VertexOutput {
  @location(0) uv: vec2<f32>,
  @builtin(position) position: vec4<f32>
}

@group(0)
@binding(0)
var tex: texture_2d<f32>;

@group(0)
@binding(1)
var samp: sampler;

@group(1)
@binding(0)
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
