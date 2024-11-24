use wgpu as w;

#[derive(Debug)]
/// wgpu で使うテクスチャ
pub struct WgpuTexture {
    pub(crate) texture: w::Texture,
    pub(crate) view: w::TextureView,
}

impl WgpuTexture {
    pub fn from_image(
        device: &w::Device,
        queue: &w::Queue,
        image: &image::RgbaImage,
        label: Option<&str>,
    ) -> Self {
        let (width, height) = image.dimensions();

        let size = w::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            texture.as_image_copy(),
            image,
            w::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(width * 4),
                rows_per_image: Some(height),
            },
            size,
        );

        let view = texture.create_view(&Default::default());

        Self { texture, view }
    }

    pub fn width(&self) -> u32 {
        self.texture.width()
    }

    pub fn height(&self) -> u32 {
        self.texture.height()
    }

    pub(crate) fn bind_group_layout(
        device: &w::Device,
        label: Option<&str>,
        texture_binding: u32,
        sampler_binding: u32,
    ) -> w::BindGroupLayout {
        device.create_bind_group_layout(&w::BindGroupLayoutDescriptor {
            label,
            entries: &[
                w::BindGroupLayoutEntry {
                    binding: texture_binding,
                    visibility: w::ShaderStages::FRAGMENT,
                    ty: w::BindingType::Texture {
                        sample_type: w::TextureSampleType::Float { filterable: true },
                        view_dimension: w::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                w::BindGroupLayoutEntry {
                    binding: sampler_binding,
                    visibility: w::ShaderStages::FRAGMENT,
                    ty: w::BindingType::Sampler(w::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        })
    }

    pub(crate) fn create_bind_group(
        &self,
        device: &w::Device,
        label: Option<&str>,
        layout: &w::BindGroupLayout,
        sampler: &w::Sampler,
        texture_binding: u32,
        sampler_binding: u32,
    ) -> w::BindGroup {
        device.create_bind_group(&w::BindGroupDescriptor {
            label,
            layout,
            entries: &[
                w::BindGroupEntry {
                    binding: texture_binding,
                    resource: w::BindingResource::TextureView(&self.view),
                },
                w::BindGroupEntry {
                    binding: sampler_binding,
                    resource: w::BindingResource::Sampler(sampler),
                },
            ],
        })
    }
}
