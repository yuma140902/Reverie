use newtype_ops::newtype_ops;

pub use nalgebra;

#[macro_use]
mod macros;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Deg<T>(pub T);

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Rad<T>(pub T);

newtype_ops! { { [Deg<f64>] [Deg<f32>] [Rad<f64>] [Rad<f32>] } {add sub} {:=} {^&}Self {^&}Self }
newtype_ops! { { [Deg<f64>] [Deg<f32>] [Rad<f64>] [Rad<f32>] } {neg} {:=} {^&}Self }
newtype_ops! { { [Deg<f64>] [Rad<f64>] } {mul div} {:=} {^&}Self {^&}f64 }
newtype_ops! { { [Deg<f32>] [Rad<f32>] } {mul div} {:=} {^&}Self {^&}f32 }
derive_into!(Deg<f64>, f64);
derive_into!(Deg<f32>, f32);
derive_into!(Rad<f64>, f64);
derive_into!(Rad<f32>, f32);
derive_approx!(Deg<f64>, f64);
derive_approx!(Deg<f32>, f32);
derive_approx!(Rad<f64>, f64);
derive_approx!(Rad<f32>, f32);

impl_deg!(f64);
impl_deg!(f32);

impl_rad!(f64);
impl_rad!(f32);

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn deg_op_add() {
        assert_relative_eq!(Deg(30_f32) + Deg(40_f32), Deg(70_f32));
        assert_relative_eq!(Deg(70_f32) + Deg(40_f32), Deg(110_f32));
        assert_relative_eq!(Deg(180_f32) + Deg(200_f32), Deg(380_f32));
    }

    #[test]
    fn deg_op_add_asign() {
        let mut a = Deg(10_f32);
        a += Deg(20_f32);
        assert_relative_eq!(a, Deg(30_f32));
    }

    #[test]
    fn deg_op_sub() {
        assert_relative_eq!(Deg(40_f32) - Deg(30_f32), Deg(10_f32));
        assert_relative_eq!(Deg(30_f32) - Deg(30_f32), Deg(0_f32));
        assert_relative_eq!(Deg(30_f32) - Deg(40_f32), Deg(-10_f32));
    }

    #[test]
    fn deg_op_sub_assign() {
        let mut a = Deg(10_f32);
        a -= Deg(10_f32);
        assert_relative_eq!(a, Deg(0_f32));
    }

    #[test]
    fn deg_op_neg() {
        assert_relative_eq!(-Deg(10_f32), Deg(-10_f32));
    }

    #[test]
    fn deg_op_mul() {
        assert_relative_eq!(Deg(10_f32) * 2_f32, Deg(20_f32));
    }

    #[test]
    fn deg_op_div() {
        assert_relative_eq!(Deg(10_f32) / 2_f32, Deg(5_f32));
    }

    #[test]
    fn deg_to_rad() {
        assert_relative_eq!(Deg(0_f32).to_rad(), Rad(0_f32));
        assert_relative_eq!(Deg(90_f32).to_rad(), Rad(std::f32::consts::PI / 2_f32));
    }

    #[test]
    fn deg_sin() {
        assert_relative_eq!(Deg(0_f32).sin(), 0_f32);
        assert_relative_eq!(Deg(90_f32).sin(), 1_f32);
    }

    #[test]
    fn deg_cos() {
        assert_relative_eq!(Deg(0_f32).cos(), 1_f32);
        assert_relative_eq!(Deg(90_f32).cos(), 0_f32);
    }

    #[test]
    fn deg_normalized() {
        assert_relative_eq!(Deg(170_f32).normalized(), Deg(170_f32));
        assert_relative_eq!(Deg(180_f32).normalized(), Deg(-180_f32));
        assert_relative_eq!(Deg(190_f32).normalized(), Deg(-170_f32));
        assert_relative_eq!(Deg(-170_f32).normalized(), Deg(-170_f32));
        assert_relative_eq!(Deg(-180_f32).normalized(), Deg(-180_f32));
        assert_relative_eq!(Deg(-190_f32).normalized(), Deg(170_f32));
    }

    #[test]
    fn rad_op_add() {
        assert_relative_eq!(Rad(30_f32) + Rad(40_f32), Rad(70_f32));
        assert_relative_eq!(Rad(70_f32) + Rad(40_f32), Rad(110_f32));
        assert_relative_eq!(Rad(180_f32) + Rad(200_f32), Rad(380_f32));
    }

    #[test]
    fn rad_op_add_asign() {
        let mut a = Rad(10_f32);
        a += Rad(20_f32);
        assert_relative_eq!(a, Rad(30_f32));
    }

    #[test]
    fn rad_op_sub() {
        assert_relative_eq!(Rad(40_f32) - Rad(30_f32), Rad(10_f32));
        assert_relative_eq!(Rad(30_f32) - Rad(30_f32), Rad(0_f32));
        assert_relative_eq!(Rad(30_f32) - Rad(40_f32), Rad(-10_f32));
    }

    #[test]
    fn rad_op_sub_assign() {
        let mut a = Rad(10_f32);
        a -= Rad(10_f32);
        assert_relative_eq!(a, Rad(0_f32));
    }

    #[test]
    fn rad_op_neg() {
        assert_relative_eq!(-Rad(10_f32), Rad(-10_f32));
    }

    #[test]
    fn rad_op_mul() {
        assert_relative_eq!(Rad(10_f32) * 2_f32, Rad(20_f32));
    }

    #[test]
    fn rad_op_div() {
        assert_relative_eq!(Rad(10_f32) / 2_f32, Rad(5_f32));
    }

    #[test]
    fn rad_to_deg() {
        assert_relative_eq!(Rad(0_f32).to_deg(), Deg(0_f32));
        assert_relative_eq!(Rad(std::f32::consts::PI / 2_f32).to_deg(), Deg(90_f32));
    }

    #[test]
    fn rad_sin() {
        assert_relative_eq!(Rad(0_f32).sin(), 0_f32);
        assert_relative_eq!(Rad(std::f32::consts::PI / 2_f32).sin(), 1_f32);
    }

    #[test]
    fn rad_cos() {
        assert_relative_eq!(Rad(0_f32).cos(), 1_f32);
        assert_relative_eq!(Rad(std::f32::consts::PI / 2_f32).cos(), 0_f32);
    }

    #[test]
    fn rad_normalized() {
        use std::f32::consts::PI;
        assert_relative_eq!(Rad(PI * 0.9).normalized(), Rad(PI * 0.9));
        assert_relative_eq!(Rad(PI * 1.0).normalized(), Rad(-PI * 1.0));
        assert_relative_eq!(Rad(PI * 1.1).normalized(), Rad(-PI * 0.9));
        assert_relative_eq!(Rad(-PI * 0.9).normalized(), Rad(-PI * 0.9));
        assert_relative_eq!(Rad(-PI * 1.0).normalized(), Rad(-PI * 1.0));
        assert_relative_eq!(Rad(-PI * 1.1).normalized(), Rad(PI * 0.9));
    }
}
