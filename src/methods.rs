use crate::Fraction;

impl Fraction {
  pub fn to_dec_frac(&self) -> f64 {
    if self.1 == 0 {
      println!("\n\x1b[1;31mDIVIDING BY ZERO!!!\x1b[0m");
      std::process::exit(1);
    }
    (self.0 as f64) / (self.1 as f64)
  }
}
