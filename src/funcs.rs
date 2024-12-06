pub fn gcd(mut num1: i64, mut num2: i64) -> i64 {
  let mut r: i64;
  while num2 > 0 {
    r = num1 % num2;
    num1 = num2;
    num2 = r;
  }
  num1
}

pub fn lcm(num1: i64, num2: i64) -> i64 {
  (num1 * num2).abs() / gcd(num1, num2)
}
