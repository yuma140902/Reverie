use newtype_ops::newtype_ops;

#[macro_use]
mod macros;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Deg(pub f64);

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Rad(pub f64);

newtype_ops! { { [Deg] [Rad] } {add sub} {:=} {^&}Self {^&}Self }
newtype_ops! { { [Deg] [Rad] } {neg} {:=} {^&}Self }
newtype_ops! { { [Deg] [Rad] } {mul div} {:=} {^&}Self {^&}f64 }
derive_into_f64!(Deg);
derive_into_f64!(Rad);
derive_approx!(Deg);
derive_approx!(Rad);

impl Deg {
    pub fn to_rad(&self) -> Rad {
        Rad(self.0 * std::f64::consts::PI / 180.0)
    }

    pub fn sin(&self) -> f64 {
        self.to_rad().0.sin()
    }

    pub fn cos(&self) -> f64 {
        self.to_rad().0.cos()
    }
}

impl Rad {
    pub fn to_deg(&self) -> Deg {
        Deg(self.0 * 180.0 / std::f64::consts::PI)
    }

    pub fn sin(&self) -> f64 {
        self.0.sin()
    }

    pub fn cos(&self) -> f64 {
        self.0.cos()
    }
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn deg_op_add() {
        assert_relative_eq!(Deg(30.0) + Deg(40.0), Deg(70.0));
        assert_relative_eq!(Deg(70.0) + Deg(40.0), Deg(110.0));
        assert_relative_eq!(Deg(180.0) + Deg(200.0), Deg(380.0));
    }

    #[test]
    fn deg_op_add_asign() {
        let mut a = Deg(10.0);
        a += Deg(20.0);
        assert_relative_eq!(a, Deg(30.0));
    }

    #[test]
    fn deg_op_sub() {
        assert_relative_eq!(Deg(40.0) - Deg(30.0), Deg(10.0));
        assert_relative_eq!(Deg(30.0) - Deg(30.0), Deg(0.0));
        assert_relative_eq!(Deg(30.0) - Deg(40.0), Deg(-10.0));
    }

    #[test]
    fn deg_op_sub_assign() {
        let mut a = Deg(10.0);
        a -= Deg(10.0);
        assert_relative_eq!(a, Deg(0.0));
    }

    #[test]
    fn deg_op_neg() {
        assert_relative_eq!(-Deg(10.0), Deg(-10.0));
    }

    #[test]
    fn deg_op_mul() {
        assert_relative_eq!(Deg(10.0) * 2.0, Deg(20.0));
    }

    #[test]
    fn deg_op_div() {
        assert_relative_eq!(Deg(10.0) / 2.0, Deg(5.0));
    }

    #[test]
    fn deg_to_rad() {
        assert_relative_eq!(Deg(0.0).to_rad(), Rad(0.0));
        assert_relative_eq!(Deg(90.0).to_rad(), Rad(std::f64::consts::PI / 2.0));
    }

    #[test]
    fn deg_sin() {
        assert_relative_eq!(Deg(0.0).sin(), 0.0);
        assert_relative_eq!(Deg(90.0).sin(), 1.0);
    }

    #[test]
    fn deg_cos() {
        assert_relative_eq!(Deg(0.0).cos(), 1.0);
        assert_relative_eq!(Deg(90.0).cos(), 0.0);
    }

    #[test]
    fn rad_op_add() {
        assert_relative_eq!(Rad(30.0) + Rad(40.0), Rad(70.0));
        assert_relative_eq!(Rad(70.0) + Rad(40.0), Rad(110.0));
        assert_relative_eq!(Rad(180.0) + Rad(200.0), Rad(380.0));
    }

    #[test]
    fn rad_op_add_asign() {
        let mut a = Rad(10.0);
        a += Rad(20.0);
        assert_relative_eq!(a, Rad(30.0));
    }

    #[test]
    fn rad_op_sub() {
        assert_relative_eq!(Rad(40.0) - Rad(30.0), Rad(10.0));
        assert_relative_eq!(Rad(30.0) - Rad(30.0), Rad(0.0));
        assert_relative_eq!(Rad(30.0) - Rad(40.0), Rad(-10.0));
    }

    #[test]
    fn rad_op_sub_assign() {
        let mut a = Rad(10.0);
        a -= Rad(10.0);
        assert_relative_eq!(a, Rad(0.0));
    }

    #[test]
    fn rad_op_neg() {
        assert_relative_eq!(-Rad(10.0), Rad(-10.0));
    }

    #[test]
    fn rad_op_mul() {
        assert_relative_eq!(Rad(10.0) * 2.0, Rad(20.0));
    }

    #[test]
    fn rad_op_div() {
        assert_relative_eq!(Rad(10.0) / 2.0, Rad(5.0));
    }

    #[test]
    fn rad_to_deg() {
        assert_relative_eq!(Rad(0.0).to_deg(), Deg(0.0));
        assert_relative_eq!(Rad(std::f64::consts::PI / 2.0).to_deg(), Deg(90.0));
    }

    #[test]
    fn rad_sin() {
        assert_relative_eq!(Rad(0.0).sin(), 0.0);
        assert_relative_eq!(Rad(std::f64::consts::PI / 2.0).sin(), 1.0);
    }

    #[test]
    fn rad_cos() {
        assert_relative_eq!(Rad(0.0).cos(), 1.0);
        assert_relative_eq!(Rad(std::f64::consts::PI / 2.0).cos(), 0.0);
    }
}
