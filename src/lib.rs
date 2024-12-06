mod basic_traits;
pub mod funcs;
pub mod methods;
mod ops;

pub struct Fraction(i64, i64);

impl Fraction {
  pub fn new(numerator: i64, denominator: i64) -> Fraction {
    if denominator == 0 {
      println!("\n\x1b[1;31mDenominator can't be zero!!!\x1b[0m");
      std::process::exit(1);
    }
    Fraction(numerator, denominator)
  }
}
