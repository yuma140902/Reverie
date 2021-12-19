//! OpenGLのバインディング
//! 
//! OpenGL 3.3 Core Profile
//! 
//! 機能拡張は無し

mod bindings {
  include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

pub use bindings::*;

use std::rc::Rc;
#[derive(Clone)]
/// 実体はRcなのでいくらでもクローンして良い
pub struct Gl {
  inner: Rc<bindings::Gl>,
}

impl Gl {
  pub fn load_with<F>(loadfn: F) -> Gl
  where
      F: FnMut(&'static str) -> *const types::GLvoid,
  {
      Gl {
          inner: Rc::new(bindings::Gl::load_with(loadfn)),
      }
  }
}

use std::ops::Deref;
impl Deref for Gl {
  type Target = bindings::Gl;

  fn deref(&self) -> &bindings::Gl {
      &self.inner
  }
}
