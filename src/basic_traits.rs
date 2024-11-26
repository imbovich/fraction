// I know about derive but i don't want to use it

impl Clone for crate::Fraction {
  fn clone(&self) -> Self {
    *self
  }
}

impl Copy for crate::Fraction {}
