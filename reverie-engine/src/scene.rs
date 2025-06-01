//! シーンに関するモジュール

use tracing_unwrap::ResultExt;

use crate::render::RenderingResource;

mod components;
mod entity;
mod system;

pub use components::{
    colored::ColoredComponent, sprite::SpriteComponent, transform::TransformComponent,
};
pub use entity::EntityIndex;
pub use system::{Frame, System};

#[derive(Default)]
/// シーン内には複数のエンティティが存在する。
pub struct Scene {
    pub(crate) world: hecs::World,
    systems: Vec<Box<dyn System>>,
}

impl Scene {
    pub fn new_sprite_entity(
        &mut self,
        transform: TransformComponent,
        sprite: SpriteComponent,
    ) -> EntityIndex {
        let entity = self.world.spawn((transform, sprite));
        EntityIndex(entity)
    }

    pub fn new_colored_entity(
        &mut self,
        transform: TransformComponent,
        colored: ColoredComponent,
    ) -> EntityIndex {
        let entity = self.world.spawn((transform, colored));
        EntityIndex(entity)
    }

    pub fn attach_component<C: hecs::Component + 'static>(
        &mut self,
        entity: EntityIndex,
        component: C,
    ) {
        self.world.insert_one(entity.0, component).unwrap_or_log();
    }

    pub fn register_system<S: System + 'static>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    pub fn setup(&mut self, resource: &RenderingResource<'_>) {
        for (_, sprite) in self.world.query_mut::<&mut SpriteComponent>() {
            sprite.setup(resource)
        }

        for (_, colored) in self.world.query_mut::<&mut ColoredComponent>() {
            colored.setup(resource)
        }

        for system in &mut self.systems {
            system.setup(resource);
        }
    }

    pub fn update(&mut self, frame: &Frame<'_>, resource: &RenderingResource<'_>) {
        for system in &mut self.systems {
            system.update(frame, &mut self.world, resource);
        }
    }

    pub fn render(&mut self, rp: &mut wgpu::RenderPass<'_>, resource: &RenderingResource<'_>) {
        rp.set_pipeline(&resource.colored_pipeline.pipeline);
        rp.set_bind_group(
            crate::render::colored::GROUP_TRANSFORM,
            &resource.colored_pipeline.uniform_bind_group,
            &[],
        );
        for (_, (transform, colored)) in self
            .world
            .query_mut::<(&TransformComponent, &mut ColoredComponent)>()
        {
            colored.render(rp, resource, transform);
        }

        rp.set_pipeline(&resource.sprite_pipeline.pipeline);
        rp.set_bind_group(
            crate::render::sprite::GROUP_TRANSFORM,
            &resource.sprite_pipeline.uniform_bind_group,
            &[],
        );
        for (_, (transform, sprite)) in self
            .world
            .query_mut::<(&TransformComponent, &mut SpriteComponent)>()
        {
            sprite.render(rp, resource, transform);
        }
    }
}

impl std::fmt::Debug for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scene")
            .field("#world", &self.world.len())
            .finish()
    }
}
