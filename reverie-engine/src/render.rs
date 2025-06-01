//! レンダリングに関するモジュール
use std::num::NonZeroU32;

use anyhow::Context;
use colored::ColoredRenderPipeline;
use nalgebra::{Point3, Vector3};
use sprite::SpriteRenderPipeline;
use wgpu::{self as w, util::DeviceExt};

use crate::{
    camera::{Camera, OrthographicCamera, PerspectiveCamera, Viewport},
    scene::Scene,
    texture::{TextureId, TextureRegistry},
};

use texture::WgpuTexture;

pub(crate) mod buffer;
pub(crate) mod colored;
pub(crate) mod sprite;
pub(crate) mod texture;
pub(crate) mod uniform;
pub(crate) mod vertex;

/// レンダリングを行うためのリソースをまとめた構造体
pub struct RenderingResource<'window> {
    pub transform_uniform_buffer: w::Buffer,
    pub texture_sampler: w::Sampler,
    pub sprite_pipeline: SpriteRenderPipeline,
    pub colored_pipeline: ColoredRenderPipeline,
    pub surface: w::Surface<'window>,
    pub surface_config: w::SurfaceConfiguration,
    pub device: w::Device,
    pub queue: w::Queue,
    pub texture_registry: TextureRegistry,
    pub camera: Camera,
    pub viewport: Viewport,
    pub depth_texture: WgpuTexture,
}

impl<'window> RenderingResource<'window> {
    /// 初期化する
    ///
    /// * `surface_target`: 描画対象の surface
    /// * `width`: surface の幅
    /// * `height`: surface の高さ
    pub async fn setup<S>(
        surface_target: S,
        width: NonZeroU32,
        height: NonZeroU32,
    ) -> anyhow::Result<Self>
    where
        S: Into<w::SurfaceTarget<'window>> + Send,
    {
        let (_instance, surface, surface_format, surface_config, _adapter, device, queue) =
            setup_instance_surface_adapter_device_queue(
                surface_target,
                width.into(),
                height.into(),
            )
            .await?;
        tracing::trace!(
            ?surface,
            ?surface_format,
            ?surface_config,
            ?device,
            ?queue,
            "setup_instance_surface_adapter_device_queue"
        );

        let viewport = Viewport { width, height };
        let camera = if true {
            PerspectiveCamera::new(
                &Point3::new(0.0, 0.0, -0.5),
                &Point3::new(0.0, 0.0, 0.0),
                &Vector3::new(0.0, 1.0, 0.0),
                90.0_f32.to_radians(),
                0.1,
                100.0,
            )
            .into()
        } else {
            OrthographicCamera {
                eye: Point3::new(0.0, 0.0, -0.5),
                target: Point3::new(0.0, 0.0, 0.0),
                up: Vector3::new(0.0, 1.0, 0.0),
                size: 0.5,
                z_near: 0.1,
                z_far: 100.0,
            }
            .into()
        };
        let transform_uniform_buffer = setup_uniform_buffer(&device, &camera, &viewport)?;

        let sampler = setup_sampler(&device)?;
        tracing::trace!(?sampler, "setup_sampler");

        let sprite_pipeline =
            SpriteRenderPipeline::new(&device, surface_format, &transform_uniform_buffer);
        tracing::trace!(?sprite_pipeline, "setup_render_pipeline");

        let colored_pipeline =
            ColoredRenderPipeline::new(&device, surface_format, &transform_uniform_buffer);

        let texture_registry = TextureRegistry::default();
        tracing::trace!(?texture_registry, "setup_texture_registry");

        let depth_texture =
            WgpuTexture::create_depth_texture(&device, width, height, Some("depth_texture"));

        Ok(Self {
            transform_uniform_buffer,
            texture_sampler: sampler,
            sprite_pipeline,
            colored_pipeline,
            surface,
            surface_config,
            device,
            queue,
            texture_registry,
            camera,
            viewport,
            depth_texture,
        })
    }

    pub fn resize(&mut self, width: NonZeroU32, height: NonZeroU32) {
        self.surface_config.width = width.get();
        self.surface_config.height = height.get();
        self.surface.configure(&self.device, &self.surface_config);

        self.viewport.width = width;
        self.viewport.height = height;
        let matrix = self
            .camera
            .get_matrix_world_to_render_coordinate(&self.viewport);
        self.queue.write_buffer(
            &self.transform_uniform_buffer,
            0,
            bytemuck::cast_slice(matrix.as_slice()),
        );

        self.depth_texture =
            WgpuTexture::create_depth_texture(&self.device, width, height, Some("depth_texture"));
    }

    pub fn get_texture_bind_group(&self, texture: TextureId) -> anyhow::Result<&w::BindGroup> {
        self.texture_registry.get_bind_group(texture)
    }

    pub fn render(&self, scene: &mut Scene) {
        if let Ok(surface_texture) = self.surface.get_current_texture() {
            let output = surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Main CommandEncoder"),
                });
            {
                let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("SpriteComponent Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &output,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 54.0 / 255.0,
                                g: 77.0 / 255.0,
                                b: 118.0 / 255.0,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(w::RenderPassDepthStencilAttachment {
                        view: &self.depth_texture.view,
                        depth_ops: Some(w::Operations {
                            load: w::LoadOp::Clear(1.0),
                            store: w::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                scene.render(&mut rp, self);
            }
            self.queue.submit(Some(encoder.finish()));

            surface_texture.present();
        } else {
            tracing::warn!("no surface texture");
        }
    }
}

#[tracing::instrument(level = "trace", skip(surface_target))]
async fn setup_instance_surface_adapter_device_queue<'window, S>(
    surface_target: S,
    width: u32,
    height: u32,
) -> anyhow::Result<(
    w::Instance,
    w::Surface<'window>,
    w::TextureFormat,
    w::SurfaceConfiguration,
    w::Adapter,
    w::Device,
    w::Queue,
)>
where
    S: Into<w::SurfaceTarget<'window>> + Send,
{
    let instance = w::Instance::default();

    let surface = instance
        .create_surface(surface_target)
        .context("fail: create surface")?;
    tracing::trace!(?surface, "created surface");

    let adapter = instance
        .request_adapter(&w::RequestAdapterOptions {
            power_preference: w::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .context("fail: request adapter")?;
    tracing::trace!(?adapter, "requested adapter");

    let (device, queue) = adapter
        .request_device(&w::DeviceDescriptor {
            label: Some("Main Device"),
            required_features: w::Features::empty(),
            required_limits: w::Limits::default(),
            memory_hints: w::MemoryHints::default(),
            trace: w::Trace::Off,
        })
        .await
        .context("fail: request device")?;
    tracing::trace!(?device, ?queue, "requested device and queue");

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .or_else(|| surface_caps.formats.first().copied())
        .context("fail: no surface format which supports SRGB")?;
    let config = w::SurfaceConfiguration {
        usage: w::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width,
        height,
        present_mode: wgpu::PresentMode::AutoVsync,
        desired_maximum_frame_latency: 2,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };
    surface.configure(&device, &config);

    Ok((
        instance,
        surface,
        surface_format,
        config,
        adapter,
        device,
        queue,
    ))
}

fn setup_uniform_buffer(
    device: &w::Device,
    camera: &Camera,
    viewport: &Viewport,
) -> anyhow::Result<w::Buffer> {
    let initial_matrix = camera.get_matrix_world_to_render_coordinate(viewport);
    Ok(device.create_buffer_init(&w::util::BufferInitDescriptor {
        label: Some("Pixel to Render Coordinate Matrix Buffer"),
        contents: bytemuck::cast_slice(initial_matrix.as_slice()),
        usage: w::BufferUsages::UNIFORM | w::BufferUsages::COPY_DST,
    }))
}

fn setup_sampler(device: &w::Device) -> anyhow::Result<w::Sampler> {
    Ok(device.create_sampler(&w::SamplerDescriptor {
        label: Some("Main Texture Sampler"),
        address_mode_u: w::AddressMode::ClampToEdge,
        address_mode_v: w::AddressMode::ClampToEdge,
        address_mode_w: w::AddressMode::ClampToEdge,
        mag_filter: w::FilterMode::Nearest,
        min_filter: w::FilterMode::Nearest,
        mipmap_filter: w::FilterMode::Nearest,
        ..Default::default()
    }))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BindingId {
    pub group: u32,
    pub binding: u32,
}

impl BindingId {
    pub const fn new(group: u32, binding: u32) -> Self {
        Self { group, binding }
    }
}
