use fraction::Fraction;
#[test]
fn mul_test() {
  let drob = Fraction(3, 4);
  let drob2 = Fraction(4, 3);
  assert_eq!(drob * drob2, Fraction(12, 12), "Multiplication test failed!");
}

#[test]
fn div_test() {
  assert_eq!(Fraction(3, 4) / Fraction(4, 3), Fraction(9, 16), "Division test failed!");
}

#[test]
fn clone_test() {
  let drob = Fraction(3, 4);
  let drob2 = drob.clone();

  assert_eq!(drob, drob2, "Clone test failed!");
}

#[test]
fn to_dev_frac_test() {
  assert_eq!(Fraction(3, 4).to_dec_frac(), 0.75, "Converting into decimal fraction failed!");
}
