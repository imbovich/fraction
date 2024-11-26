use crate::Fraction;

impl std::fmt::Display for Fraction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}/{}", self.0, self.1)
  }
}

impl std::fmt::Debug for Fraction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}/{}", self.0, self.1)
  }
}
impl std::ops::Mul for Fraction {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
    Fraction(self.0 * rhs.0, self.1 * rhs.1)
  }
}
impl std::ops::Div for Fraction {
  type Output = Self;
  fn div(self, rhs: Self) -> Self::Output {
    let mut temp_frac = rhs.clone();
    let temp = temp_frac.0;
    temp_frac.0 = temp_frac.1;
    temp_frac.1 = temp;
    self * temp_frac
  }
}
