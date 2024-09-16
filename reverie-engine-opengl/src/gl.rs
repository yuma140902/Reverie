//! OpenGLのバインディング
//!
//! OpenGL 3.3 Core Profile
//!
//! 機能拡張は無し

#[allow(clippy::all)]
#[allow(clippy::nursery)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

pub use bindings::*;

use std::fmt::Debug;
use std::rc::Rc;
#[derive(Clone)]
/// 実体は[`std::rc::Rc`]なのでいくらでもクローンして良い
pub struct Gl {
    inner: Rc<bindings::Gl>,
}

impl Gl {
    pub fn load_with<F>(loadfn: F) -> Self
    where
        F: FnMut(&'static str) -> *const types::GLvoid,
    {
        Self {
            inner: Rc::new(bindings::Gl::load_with(loadfn)),
        }
    }
}

impl Debug for Gl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<OpenGL Bindings>")
    }
}

use std::ops::Deref;
impl Deref for Gl {
    type Target = bindings::Gl;

    fn deref(&self) -> &bindings::Gl {
        &self.inner
    }
}
