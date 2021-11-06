//! 補間に関するモジュール

use self::types::*;

mod types;
pub use types::{Time, TimeSpan};

/// 補間をする構造体
///
/// `begin`から`end`まで、`t_total`の時間をかけて値を変化させる。
pub struct Interpolation<T> {
    begin: T,
    end: T,
    t_0: Time,
    t_total: TimeSpan,
    rate: fn(NormalizedTime) -> T,
}

impl Interpolation<f32> {
    /// 補間関数を指定してオブジェクトを作る。
    /// * `begin` - t_0の時の値
    /// * `end` - t_0 + t_total の時の値
    /// * `rate` - [0.0, 1.0]の値を受け取り、[0.0, 1.0]の値を返す関数
    pub fn new(
        begin: f32,
        end: f32,
        t_0: Time,
        t_total: TimeSpan,
        rate: fn(NormalizedTime) -> f32,
    ) -> Self {
        debug_assert_ne!(t_total, 0);
        Self {
            begin,
            end,
            t_0,
            t_total,
            rate,
        }
    }

    /// 線形補間
    pub fn new_lerp(begin: f32, end: f32, t_0: Time, t_total: TimeSpan) -> Self {
        Self::new(begin, end, t_0, t_total, |t| t)
    }

    /// 三次関数を使ってease-in/ease-outな補間をする
    pub fn new_cubic_ease_in_out(begin: f32, end: f32, t_0: Time, t_total: TimeSpan) -> Self {
        Self::new(begin, end, t_0, t_total, |t| -2.0 * t * t * t + 3.0 * t * t)
    }

    pub fn value(&self, t: Time) -> f32 {
        let t_normalized: NormalizedTime = ((t - self.t_0) as f32 / self.t_total as f32).clamp(0.0, 1.0);
        let rate = (self.rate)(t_normalized);
        self.begin * (1.0f32 - rate) + self.end * rate
    }
}

#[cfg(test)]
mod tests {
    // cargo test -- --nocapture --test-threads=1
    // で標準出力を表示できる
    use super::*;

    fn print_spaces(n: i32) {
        for _i in 0..n {
            print!(" ");
        }
    }

    fn display_interpolation(interpolation: &Interpolation<f32>) {
        let mut time = interpolation.t_0;
        loop {
            let value = interpolation.value(time);
            print!("{:>2.0}:", value);
            print_spaces(value as i32);
            println!("*");
            time += 1;
            if time > interpolation.t_0 + (interpolation.t_total as i64) {
                break;
            }
        }
    }

    #[test]
    fn display_lerp() {
        println!("Lerp");
        let lerp = Interpolation::new_lerp(0.0, 50.0, 5, 20);
        display_interpolation(&lerp);
    }

    #[test]
    fn display_cubic() {
        println!("Cubic");
        let cubic = Interpolation::new_cubic_ease_in_out(0.0, 50.0, 0, 20);
        display_interpolation(&cubic);
    }
}
