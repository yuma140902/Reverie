macro_rules! derive_into {
    ($target:ty, $into:ty) => {
        impl<'a> From<&'a $target> for $into {
            fn from(value: &'a $target) -> $into {
                value.0
            }
        }

        impl From<$target> for $into {
            fn from(value: $target) -> $into {
                value.0
            }
        }
    };
}

macro_rules! impl_deg {
    ($float:ident) => {
        impl Deg<$float> {
            pub fn to_rad(&self) -> Rad<$float> {
                Rad(self.0.to_radians())
            }

            pub fn sin(&self) -> $float {
                self.to_rad().0.sin()
            }

            pub fn cos(&self) -> $float {
                self.to_rad().0.cos()
            }

            pub fn normalized(&self) -> Deg<$float> {
                let mut angle = (self.0 + 180.0) % 360.0 - 180.0;
                if angle < -180.0 {
                    angle += 360.0;
                }
                Deg(angle)
            }
        }
    };
}

macro_rules! impl_rad {
    ($float:ident) => {
        impl Rad<$float> {
            pub fn to_deg(&self) -> Deg<$float> {
                Deg(self.0.to_degrees())
            }

            pub fn sin(&self) -> $float {
                self.0.sin()
            }

            pub fn cos(&self) -> $float {
                self.0.cos()
            }

            pub fn normalized(&self) -> Rad<$float> {
                use std::$float::consts::PI;
                let mut angle = (self.0 + PI) % (2.0 * PI) - PI;
                if angle < -PI {
                    angle += 2.0 * PI;
                }
                Rad(angle)
            }
        }
    };
}

macro_rules! derive_approx {
    ($target:ty, $float:ident) => {
        impl approx::AbsDiffEq for $target {
            type Epsilon = <$float as approx::AbsDiffEq>::Epsilon;

            fn default_epsilon() -> Self::Epsilon {
                $float::default_epsilon()
            }

            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                <&Self as Into<$float>>::into(self).abs_diff_eq(&other.into(), epsilon)
            }
        }

        impl approx::RelativeEq for $target {
            fn default_max_relative() -> Self::Epsilon {
                $float::default_max_relative()
            }

            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                <&Self as Into<$float>>::into(self).relative_eq(
                    &other.into(),
                    epsilon,
                    max_relative,
                )
            }
        }

        impl approx::UlpsEq for $target {
            fn default_max_ulps() -> u32 {
                $float::default_max_ulps()
            }

            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                <&Self as Into<$float>>::into(self).ulps_eq(&other.into(), epsilon, max_ulps)
            }
        }
    };
}
