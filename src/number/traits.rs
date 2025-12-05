use std::ops::{Add, Sub, Mul, Div};
use super::Number;

impl Add for Number {
    type Output = Number;
    fn add(self, rhs: Number) -> Number {
        self.add(&rhs)
    }
}

impl<'a> Add<&'a Number> for Number {
    type Output = Number;
    fn add(self, rhs: &'a Number) -> Number {
        Number::add(&self, rhs)
    }
}

impl Sub for Number {
    type Output = Number;
    fn sub(self, rhs: Number) -> Number {
        self.sub(&rhs)
    }
}

impl<'a> Sub<&'a Number> for Number {
    type Output = Number;
    fn sub(self, rhs: &'a Number) -> Number {
        Number::sub(&self, rhs)
    }
}

impl Mul for Number {
    type Output = Number;
    fn mul(self, rhs: Number) -> Number {
        self.mul(&rhs)
    }
}

impl<'a> Mul<&'a Number> for Number {
    type Output = Number;
    fn mul(self, rhs: &'a Number) -> Number {
        Number::mul(&self, rhs)
    }
}

impl Div for Number {
    type Output = Option<Number>;
    fn div(self, rhs: Number) -> Option<Number> {
        self.div(&rhs)
    }
}

impl<'a> Div<&'a Number> for Number {
    type Output = Option<Number>;
    fn div(self, rhs: &'a Number) -> Option<Number> {
        Number::div(&self, rhs)
    }
}
