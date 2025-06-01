use wgpu as w;

pub fn create_mat4_bind_group(
    device: &w::Device,
    mat4_buffer: &w::Buffer,
    binding: u32,
) -> (w::BindGroupLayout, w::BindGroup) {
    let layout = device.create_bind_group_layout(&w::BindGroupLayoutDescriptor {
        label: Some("Main Bind Group Layout"),
        entries: &[w::BindGroupLayoutEntry {
            binding,
            visibility: w::ShaderStages::VERTEX,
            ty: w::BindingType::Buffer {
                ty: w::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: w::BufferSize::new(size_of::<[f32; 4 * 4]>() as u64),
            },
            count: None,
        }],
    });

    let bind_group = device.create_bind_group(&w::BindGroupDescriptor {
        label: Some("Main Bind Group"),
        layout: &layout,
        entries: &[w::BindGroupEntry {
            binding,
            resource: mat4_buffer.as_entire_binding(),
        }],
    });

    (layout, bind_group)
}
