use crate::Fraction;

impl Fraction {
  pub fn to_dec_frac(&self) -> f64 {
    if self.1 == 0 {
      println!("\n\x1b[1;31mDIVIDING BY ZERO!!!\x1b[0m");
      std::process::exit(1);
    }
    (self.0 as f64) / (self.1 as f64)
  }

  pub fn reduce(&mut self) {
    let mut a: i64 = self.0;
    let mut b: i64 = self.1;
    let mut r: i64;
    while b > 0 {
      r = a % b;
      a = b;
      b = r;
    }
    self.0 = self.0 / a;
    self.1 = self.1 / a;
  }

  pub fn gcd_denom(&self, other: &Fraction) -> i64 {
    let mut denom1: i64 = self.1;
    let mut denom2: i64 = other.1;
    let mut r: i64;
    while denom2 > 0 {
      r = denom1 % denom2;
      denom1 = denom2;
      denom2 = r;
    }
    denom1
  }

  pub fn gcd_numer(&self, other: &Fraction) -> i64 {
    let mut numer1: i64 = self.1;
    let mut numer2: i64 = other.1;
    let mut r: i64;
    while numer2 > 0 {
      r = numer1 % numer2;
      numer1 = numer2;
      numer2 = r;
    }
    numer1
  }
}
