//! [`crate::vao::Vao`]の設定

/// [`crate::vao::Vao`]の設定
#[derive(Debug, Clone, Copy)]
pub struct VaoConfig {
    pub(crate) depth_test: bool,
    pub(crate) blend: bool,
    pub(crate) wireframe: bool,
    pub(crate) culling: bool,
}

/// [`VaoConfig`]のビルダー
#[derive(Debug)]
pub struct VaoConfigBuilder {
    depth_test: bool,
    blend: bool,
    wireframe: bool,
    culling: bool,
}

impl Default for VaoConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl VaoConfigBuilder {
    pub const fn new() -> Self {
        Self {
            depth_test: true,
            blend: true,
            wireframe: false,
            culling: true,
        }
    }

    pub const fn build(self) -> VaoConfig {
        VaoConfig {
            depth_test: self.depth_test,
            blend: self.blend,
            wireframe: self.wireframe,
            culling: self.culling,
        }
    }

    pub const fn depth_test(mut self, value: bool) -> Self {
        self.depth_test = value;
        self
    }

    pub const fn blend(mut self, value: bool) -> Self {
        self.blend = value;
        self
    }

    pub const fn wireframe(mut self, value: bool) -> Self {
        self.wireframe = value;
        self
    }

    pub const fn culling(mut self, value: bool) -> Self {
        self.culling = value;
        self
    }
}
