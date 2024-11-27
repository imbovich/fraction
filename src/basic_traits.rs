// I know about derive but i don't want to use it
use crate::Fraction;

impl Clone for Fraction {
  fn clone(&self) -> Self {
    *self
  }
}

impl Copy for Fraction {}

impl std::fmt::Display for Fraction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}/{}", self.0, self.1)
  }
}

impl std::fmt::Debug for Fraction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.0, self.1)
  }
}

impl PartialEq for Fraction {
  fn eq(&self, other: &Self) -> bool {
    let mut first = self.clone();
    let mut second = other.clone();

    first.reduce();
    second.reduce();

    (first.0 == second.0) && (first.1 == second.1)
  }
}
