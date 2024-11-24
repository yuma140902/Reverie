//! テクスチャに関するモジュール
use anyhow::Context;
use etagere::{size2, AtlasAllocator};
use image::{GenericImage, RgbaImage};
use slotmap::SlotMap;

use crate::wgpu_wrapper::texture::WgpuTexture;

#[derive(Debug)]
/// テクスチャ
struct Texture {
    data: TextureData,
    usage: TextureUsage,
    label: Option<String>,
}

impl Texture {
    pub fn width(&self) -> u32 {
        match &self.data {
            TextureData::Cpu(image) => image.width(),
            TextureData::Gpu(texture, _) => texture.width(),
        }
    }

    pub fn height(&self) -> u32 {
        match &self.data {
            TextureData::Cpu(image) => image.height(),
            TextureData::Gpu(texture, _) => texture.height(),
        }
    }

    pub fn send_to_gpu(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_layout: &wgpu::BindGroupLayout,
        sampler: &wgpu::Sampler,
        texture_binding: u32,
        sampler_binding: u32,
    ) {
        match &self.data {
            TextureData::Cpu(image) => {
                let texture = WgpuTexture::from_image(device, queue, image, self.label.as_deref());
                let label = self.label.as_deref().map(|s| format!("{s} bind_group"));
                let bind_group = texture.create_bind_group(
                    device,
                    label.as_deref(),
                    bind_group_layout,
                    sampler,
                    texture_binding,
                    sampler_binding,
                );
                self.data = TextureData::Gpu(texture, bind_group);
            }
            TextureData::Gpu(_, _) => {}
        }
    }
}

#[derive(Debug)]
/// テクスチャのデータ
///
/// テクスチャがCPU上にある場合は[`TextureData::Cpu`]、GPU上にある場合は[`TextureData::Gpu`]となる。
enum TextureData {
    Cpu(Box<RgbaImage>),
    Gpu(WgpuTexture, wgpu::BindGroup),
}

/// テクスチャの使用方法
///
/// 1つのテクスチャを使いまわす場合は[`TextureUsage::Single`]、複数のテクスチャをアトラステクスチャとして使う場合は[`TextureUsage::Atlas`]となる。
enum TextureUsage {
    Single,
    Atlas(AtlasAllocator),
}

impl std::fmt::Debug for TextureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single => write!(f, "Single"),
            Self::Atlas(_) => write!(f, "Atlas(AtlasAllocator{{*}})"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// [`TextureRegistry`]に登録されたテクスチャを指す識別子
///
/// 単一のテクスチャを指す場合は[`TextureId::Single`]、アトラステクスチャのアロケーションを指す場合は[`TextureId::Atlas`]となる。
pub enum TextureId {
    Single(TextureIndex),
    Atlas(Allocation),
}

impl From<TextureIndex> for TextureId {
    fn from(index: TextureIndex) -> Self {
        Self::Single(index)
    }
}

impl From<Allocation> for TextureId {
    fn from(allocation: Allocation) -> Self {
        Self::Atlas(allocation)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// [`TextureRegistry`]に登録された単一のテクスチャを指すインデックス
pub struct TextureIndex(slotmap::DefaultKey);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// [`TextureRegistry`]に登録されたアトラステクスチャ内のアロケーションを指す識別子
pub struct Allocation(TextureIndex, etagere::AllocId);

#[derive(Debug, Default)]
/// テクスチャを管理するレジストリ
pub struct TextureRegistry {
    arena: SlotMap<slotmap::DefaultKey, Texture>,
}

impl TextureRegistry {
    pub fn new_texture(&mut self, image: RgbaImage, label: Option<String>) -> TextureIndex {
        let texture = Texture {
            data: TextureData::Cpu(Box::new(image)),
            usage: TextureUsage::Single,
            label,
        };
        TextureIndex(self.arena.insert(texture))
    }

    pub fn create_altas_texture(
        &mut self,
        width: u32,
        height: u32,
        label: Option<String>,
    ) -> TextureIndex {
        let image = Box::new(RgbaImage::new(width, height));
        let texture = Texture {
            data: TextureData::Cpu(image),
            usage: TextureUsage::Atlas(AtlasAllocator::new(size2(width as i32, height as i32))),
            label,
        };
        TextureIndex(self.arena.insert(texture))
    }

    pub fn allocate_sub_image(
        &mut self,
        index: TextureIndex,
        sub_image: RgbaImage,
    ) -> anyhow::Result<Allocation> {
        let texture = self
            .arena
            .get_mut(index.0)
            .with_context(|| format!("no such texture: {:?}", index))?;
        if let Texture {
            data: TextureData::Cpu(image),
            usage: TextureUsage::Atlas(allocator),
            ..
        } = texture
        {
            let allocation = allocator
                .allocate(size2(sub_image.width() as i32, sub_image.height() as i32))
                .context("failed to allocate")?;
            let rect = allocation.rectangle;
            image
                .copy_from(&sub_image, rect.min.x as u32, rect.min.y as u32)
                .context("failed to copy sub_image")?;
            Ok(Allocation(index, allocation.id))
        } else {
            anyhow::bail!("invalid texture is not for atlas or texture is not on CPU")
        }
    }

    pub fn get_uv(&self, id: TextureId) -> anyhow::Result<(f32, f32, f32, f32)> {
        match id {
            TextureId::Single(_) => Ok((0.0, 0.0, 1.0, 1.0)),
            TextureId::Atlas(allocation) => {
                let texture = self
                    .arena
                    .get(allocation.0 .0)
                    .with_context(|| format!("no such texture: {:?}", allocation.0))?;
                if let Texture {
                    usage: TextureUsage::Atlas(allocator),
                    ..
                } = texture
                {
                    let width = texture.width() as f32;
                    let height = texture.height() as f32;
                    let rect = allocator.get(allocation.1);

                    let min_u = rect.min.x as f32 / width;
                    let min_v = rect.min.y as f32 / height;
                    let max_u = rect.max.x as f32 / width;
                    let max_v = rect.max.y as f32 / height;
                    Ok((min_u, min_v, max_u, max_v))
                } else {
                    anyhow::bail!("texture is not for atlas")
                }
            }
        }
    }

    /// CPU 上のテクスチャを GPU に送信する
    pub fn send_all_to_gpu(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_layout: &wgpu::BindGroupLayout,
        sampler: &wgpu::Sampler,
        texture_binding: u32,
        sampler_binding: u32,
    ) {
        for (_, texture) in self.arena.iter_mut() {
            texture.send_to_gpu(
                device,
                queue,
                bind_group_layout,
                sampler,
                texture_binding,
                sampler_binding,
            );
        }
    }

    pub fn get_bind_group(&self, id: TextureId) -> anyhow::Result<&wgpu::BindGroup> {
        match id {
            TextureId::Single(index) => {
                let texture = self
                    .arena
                    .get(index.0)
                    .with_context(|| format!("no such texture: {:?}", index))?;
                if let Texture {
                    data: TextureData::Gpu(_, bind_group),
                    ..
                } = texture
                {
                    Ok(bind_group)
                } else {
                    anyhow::bail!("texture is not on GPU")
                }
            }
            TextureId::Atlas(allocation) => {
                let texture = self
                    .arena
                    .get(allocation.0 .0)
                    .with_context(|| format!("no such texture: {:?}", allocation.0))?;
                if let Texture {
                    data: TextureData::Gpu(_, bind_group),
                    ..
                } = texture
                {
                    Ok(bind_group)
                } else {
                    anyhow::bail!("texture is not on GPU")
                }
            }
        }
    }
}
