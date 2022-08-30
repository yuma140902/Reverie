use std::ffi::c_void;

use crate::{gl::Gl, Window};

pub trait ContextBackend {
    fn new(window: &Window) -> Self;
    fn get_proc_address(&self, symbol: &str) -> *const c_void;
    /// この[`Context`]を描画先として設定する
    fn make_current(&self);
    fn make_not_current(&self);
    fn swap_buffers(&self);
}

#[cfg(feature = "raw_gl_context")]
impl ContextBackend for raw_gl_context::GlContext {
    fn new(window: &Window) -> Self {
        let context =
            raw_gl_context::GlContext::create(&window.window, raw_gl_context::GlConfig::default())
                .unwrap();
        context
    }
    fn get_proc_address(&self, symbol: &str) -> *const c_void {
        self.get_proc_address(symbol)
    }

    fn make_current(&self) {
        self.make_current()
    }

    fn make_not_current(&self) {
        self.make_not_current()
    }

    fn swap_buffers(&self) {
        self.swap_buffers()
    }
}

#[cfg(feature = "glutin")]
impl ContextBackend for glutin::RawContext<glutin::PossiblyCurrent> {
    #[cfg(target_os = "windows")]
    fn new(window: &Window) -> Self {
        use glutin::platform::windows::RawContextExt;
        use winit::platform::windows::WindowExtWindows;
        let hwnd = window.window.hwnd();
        let raw_context = unsafe { glutin::ContextBuilder::new().build_raw_context(hwnd) }.unwrap();
        let raw_context = unsafe { raw_context.make_current() }.unwrap();
        raw_context
    }

    #[cfg(not(target_os = "windows"))]
    fn new(window: &Window) -> Self {
        panic!("glutin rawcontext is not implemented for this platform");
    }

    fn get_proc_address(&self, symbol: &str) -> *const c_void {
        self.get_proc_address(symbol)
    }

    fn make_current(&self) {
        // todo
    }

    fn make_not_current(&self) {
        // todo
    }

    fn swap_buffers(&self) {
        self.swap_buffers().unwrap();
    }
}

pub struct Context<C: ContextBackend> {
    backend: C,
    gl: Gl,
}

impl<C: ContextBackend> Context<C> {
    pub fn new(window: &Window) -> Self {
        let backend = C::new(window);
        let gl = Gl::load_with(|symbol| backend.get_proc_address(symbol) as *const _);

        Self { backend, gl }
    }
    pub fn gl(&self) -> Gl {
        Gl::clone(&self.gl)
    }

    /// この[`Context`]を描画先として設定する
    pub fn make_current(&self) {
        self.backend.make_current();
    }

    pub fn make_not_current(&self) {
        self.backend.make_not_current();
    }

    pub fn swap_buffers(&self) {
        self.backend.swap_buffers();
    }
}
