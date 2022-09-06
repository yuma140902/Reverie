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
}
