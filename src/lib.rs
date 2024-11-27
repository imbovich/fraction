mod basic_traits;
pub mod methods;
mod ops;

//Temporary solution
// TODO make adekvatniy PartialEq

#[derive(PartialEq)]

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
