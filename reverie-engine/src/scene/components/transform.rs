use nalgebra::{Affine3, Isometry3, Matrix4, Scale3, Translation3, UnitQuaternion};

#[derive(Debug)]
/// エンティティの位置、回転、拡大縮小を表すコンポーネント
pub struct TransformComponent {
    pub translation: Translation3<f32>,
    pub rotation: UnitQuaternion<f32>,
    pub scale: Scale3<f32>,
}

impl Default for TransformComponent {
    fn default() -> Self {
        Self {
            translation: Translation3::identity(),
            rotation: UnitQuaternion::identity(),
            scale: Scale3::identity(),
        }
    }
}

impl TransformComponent {
    pub const fn new(
        translation: Translation3<f32>,
        scale: Scale3<f32>,
        rotation: UnitQuaternion<f32>,
    ) -> Self {
        Self {
            translation,
            scale,
            rotation,
        }
    }

    pub fn with_translation(translation: Translation3<f32>) -> Self {
        Self {
            translation,
            ..Default::default()
        }
    }

    pub fn with_translation_and_scale(translation: Translation3<f32>, scale: Scale3<f32>) -> Self {
        Self {
            translation,
            scale,
            ..Default::default()
        }
    }

    pub fn with_translation_and_rotation(
        translation: Translation3<f32>,
        rotation: UnitQuaternion<f32>,
    ) -> Self {
        Self {
            translation,
            rotation,
            ..Default::default()
        }
    }

    pub fn to_affine3(&self) -> Affine3<f32> {
        Affine3::from_matrix_unchecked(
            self.to_isometry3().to_homogeneous()
                * Matrix4::new_nonuniform_scaling(&self.scale.vector),
        )
    }

    pub fn to_isometry3(&self) -> Isometry3<f32> {
        Isometry3::from_parts(self.translation, self.rotation)
    }
}
