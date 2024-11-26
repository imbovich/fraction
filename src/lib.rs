mod basic_traits;
pub mod methods;
mod ops;

//Temporary solution
// TODO make adekvatniy PartialEq

#[derive(PartialEq)]

pub struct Fraction(pub i64, pub i64);
