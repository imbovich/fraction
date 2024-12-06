use crate::funcs::lcm;
use crate::Fraction;

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

impl std::ops::Add for Fraction {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    let mut temp_frac1 = self;
    let mut temp_frac2 = rhs;
    let lcm = lcm(self.1, rhs.1);
    let first_multiplier = lcm / self.1;
    let second_multiplier = lcm / rhs.1;
    temp_frac1.0 = temp_frac1.0 * first_multiplier;
    temp_frac2.0 = temp_frac2.0 * second_multiplier;
    Fraction::new(temp_frac1.0 + temp_frac2.0, lcm)
  }
}

impl std::ops::Sub for Fraction {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output {
    let mut temp_frac1 = self;
    let mut temp_frac2 = rhs;
    let lcm = lcm(self.1, rhs.1);
    let first_multiplier = lcm / self.1;
    let second_multiplier = lcm / rhs.1;
    temp_frac1.0 = temp_frac1.0 * first_multiplier;
    temp_frac2.0 = temp_frac2.0 * second_multiplier;
    Fraction::new(temp_frac1.0 - temp_frac2.0, lcm)
  }
}
// TODO: Оптимизировать Add и Sub
