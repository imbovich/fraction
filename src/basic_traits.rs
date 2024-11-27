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
    write!(f, "{}/{}", self.0, self.1)
  }
}
