macro_rules! derive_into_f64 {
    ($target:ty) => {
        impl<'a> Into<f64> for &'a $target {
            fn into(self) -> f64 {
                self.0
            }
        }

        impl Into<f64> for $target {
            fn into(self) -> f64 {
                self.0
            }
        }
    };
}

macro_rules! derive_approx {
    ($target:ty) => {
        impl approx::AbsDiffEq for $target {
            type Epsilon = <f64 as approx::AbsDiffEq>::Epsilon;

            fn default_epsilon() -> Self::Epsilon {
                f64::default_epsilon()
            }

            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                <&Self as Into<f64>>::into(self).abs_diff_eq(&other.into(), epsilon)
            }
        }

        impl approx::RelativeEq for $target {
            fn default_max_relative() -> Self::Epsilon {
                f64::default_max_relative()
            }

            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                <&Self as Into<f64>>::into(self).relative_eq(&other.into(), epsilon, max_relative)
            }
        }

        impl approx::UlpsEq for $target {
            fn default_max_ulps() -> u32 {
                f64::default_max_ulps()
            }

            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                <&Self as Into<f64>>::into(self).ulps_eq(&other.into(), epsilon, max_ulps)
            }
        }
    };
}
