impl crate::Fraction {
  pub fn to_dec_frac(&self) -> f64 {
    (self.0 as f64) / (self.1 as f64)
  }
}
