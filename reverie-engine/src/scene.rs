//! シーンに関するモジュール

use crate::{render::RenderingResource, scene::frame::Frame};

mod components;
pub mod frame;

pub use components::{
    colored::ColoredComponent, sprite::SpriteComponent, transform::TransformComponent,
};

#[derive(Default)]
pub struct Scene {}

impl Scene {
    pub const fn setup(&mut self, _resource: &RenderingResource<'_>) {}

    pub const fn update(&mut self, _frame: &Frame<'_>, _resource: &RenderingResource<'_>) {}

    pub fn render(&mut self, rp: &mut wgpu::RenderPass<'_>, resource: &RenderingResource<'_>) {
        rp.set_pipeline(&resource.colored_pipeline.pipeline);
        rp.set_bind_group(
            crate::render::colored::GROUP_TRANSFORM,
            &resource.colored_pipeline.uniform_bind_group,
            &[],
        );

        rp.set_pipeline(&resource.sprite_pipeline.pipeline);
        rp.set_bind_group(
            crate::render::sprite::GROUP_TRANSFORM,
            &resource.sprite_pipeline.uniform_bind_group,
            &[],
        );
    }
}

impl std::fmt::Debug for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scene").finish()
    }
}
