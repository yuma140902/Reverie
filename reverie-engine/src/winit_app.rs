//! winit のイベントループを使ったアプリケーションの実行を行うモジュール
use std::{num::NonZeroU32, sync::Arc, time::Instant};

use anyhow::Context;
use tracing_unwrap::ResultExt;
use wgpu::rwh::{HasDisplayHandle, HasWindowHandle};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalPosition,
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent},
    event_loop::ActiveEventLoop,
    window::Window,
};

use crate::{
    game::Game,
    render::{RenderingResource, sprite},
    scene::{Frame, Scene},
};

pub struct App<'window, G: Game> {
    game: G,
    scene: Option<Scene>,
    resource: Option<AppResource<'window>>,
    last_update: Instant,
    key_events: Vec<KeyEvent>,
    mouse_clicks: Vec<(ElementState, MouseButton, PhysicalPosition<f64>)>,
    mouse_wheels: Vec<(MouseScrollDelta, TouchPhase, PhysicalPosition<f64>)>,
    last_mouse_pos: PhysicalPosition<f64>,
}

impl<G: Game> App<'_, G> {
    pub fn new(game: G) -> Self {
        Self {
            game,
            scene: None,
            resource: None,
            last_update: Instant::now(),
            key_events: Vec::new(),
            mouse_clicks: Vec::new(),
            mouse_wheels: Vec::new(),
            last_mouse_pos: PhysicalPosition::new(0.0, 0.0),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn setup(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.resource.is_none() {
            let mut r = AppResource::new(event_loop).unwrap_or_log();
            let mut scene = self
                .game
                .generate_scene(&mut r.render.texture_registry)
                .context("failed: generate scene")
                .unwrap_or_log();
            r.render.texture_registry.send_all_to_gpu(
                &r.render.device,
                &r.render.queue,
                &r.render.render_pipeline.texture_bind_group_layout,
                &r.render.texture_sampler,
                sprite::BINDING_TEXTURE.binding,
                sprite::BINDING_SAMPLER.binding,
            );
            println!("{:?}", scene);
            scene.setup(&r.render);

            self.resource = Some(r);
            self.scene = Some(scene);
        }
    }

    fn update(&mut self) {
        if let (Some(r), Some(scene)) = (self.resource.as_mut(), self.scene.as_mut()) {
            let now = Instant::now();
            let frame = Frame {
                delta_time: now - self.last_update,
                now,
                key_events: self.key_events.as_slice(),
                mouse_clicks: self.mouse_clicks.as_slice(),
                mouse_wheels: self.mouse_wheels.as_slice(),
                mouse_position: self.last_mouse_pos,
            };

            scene.update(&frame, &r.render);

            self.last_update = now;
            self.key_events.clear();
            self.mouse_clicks.clear();
            self.mouse_wheels.clear();

            r.render.render(scene);
            r.window.0.request_redraw();
        }
    }
}

impl<G: Game> ApplicationHandler for App<'_, G> {
    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        if cause == winit::event::StartCause::Poll {
            if let Some(r) = self.resource.as_ref() {
                r.window.0.request_redraw();
            }
        }
    }

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.setup(event_loop);
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                self.resource = None;
            }
            WindowEvent::Resized(size) => {
                if let Some(r) = self.resource.as_mut() {
                    if let (Some(width), Some(height)) =
                        (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                    {
                        r.render.resize(width, height);
                        r.window.0.request_redraw();
                    }
                }
            }
            WindowEvent::RedrawRequested => self.update(),
            WindowEvent::KeyboardInput { event, .. } => {
                if self.resource.is_some() {
                    self.key_events.push(event);
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.last_mouse_pos = position;
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.mouse_clicks.push((state, button, self.last_mouse_pos));
            }
            WindowEvent::MouseWheel { delta, phase, .. } => {
                self.mouse_wheels.push((delta, phase, self.last_mouse_pos));
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.resource.is_none() {
            event_loop.exit();
        }
    }
}

pub struct AppResource<'window> {
    pub window: ArcWindow,
    pub render: RenderingResource<'window>,
}

impl AppResource<'_> {
    pub fn new(event_loop: &ActiveEventLoop) -> anyhow::Result<Self> {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap_or_log();
        let window = ArcWindow(Arc::new(window));
        let size = window.0.inner_size();
        let width = size
            .width
            .try_into()
            .context("error: window inner width is zero")?;
        let height = size
            .height
            .try_into()
            .context("error: window inner height is zero")?;

        let render = pollster::block_on(RenderingResource::setup(window.clone(), width, height))
            .context("failed: setup wgpu")?;

        Ok(Self { window, render })
    }
}

#[derive(Clone)]
pub struct ArcWindow(pub Arc<winit::window::Window>);

impl HasWindowHandle for ArcWindow {
    fn window_handle(&self) -> Result<wgpu::rwh::WindowHandle<'_>, wgpu::rwh::HandleError> {
        self.0.window_handle()
    }
}

impl HasDisplayHandle for ArcWindow {
    fn display_handle(&self) -> Result<wgpu::rwh::DisplayHandle<'_>, wgpu::rwh::HandleError> {
        self.0.display_handle()
    }
}
