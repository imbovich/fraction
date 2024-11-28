use fraction::Fraction;

// ТЕСТЫ ДЛЯ ВСЕГО!!! Я хочу чтобы cargo выводил много ok

#[test]
fn new_test() {
  let drob = Fraction::new(3, 4);
  assert_eq!(drob, Fraction::new(3, 4), "Creating fraction failed!");
}

#[test]
fn mul_test() {
  let drob = Fraction::new(3, 4);
  let drob2 = Fraction::new(4, 3);
  assert_eq!(drob * drob2, Fraction::new(12, 12), "Multiplication test failed!");
}

#[test]
fn div_test() {
  assert_eq!(Fraction::new(3, 4) / Fraction::new(4, 3), Fraction::new(9, 16), "Division test failed!");
}

#[test]
fn clone_test() {
  let drob = Fraction::new(3, 4);
  let drob2 = drob.clone();

  assert_eq!(drob, drob2, "Clone test failed!");
}

#[test]
fn to_dev_frac_test() {
  assert_eq!(Fraction::new(3, 4).to_dec_frac(), 0.75, "Converting into decimal fraction failed!");
}

#[test]
fn reduce_test() {
  let mut first = Fraction::new(2, 4);
  first.reduce();
  assert_eq!(first, Fraction::new(1, 2), "Reducing failed!");
  first = Fraction::new(5, 7);
  first.reduce();
  assert_eq!(first, Fraction::new(5, 7), "Irreducible fraction reduced!");
}

#[test]
fn partialeq_test() {
  assert_eq!(Fraction::new(1, 2) == Fraction::new(2, 4), true, "PartialEq incorrectly implemented!");
}

#[test]
fn gcd_denom_test() {
  let frac1 = Fraction::new(2, 4);
  let frac2 = Fraction::new(4, 8);
  assert_eq!(frac1.gcd_denom(&frac2), 4, "Incorrect gcd for denominator!");
}

#[test]
fn gcd_numer_test() {
  let frac1 = Fraction::new(2, 4);
  let frac2 = Fraction::new(4, 8);
  assert_eq!(frac1.gcd_numer(&frac2), 4, "Incorrect gcd for numerator!");
}
