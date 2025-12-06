use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Ordering;
use super::{Number, Sign};

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        // Two numbers are equal if a/b == c/d, which means a*d == b*c
        let ad = Number::mul_vec(&self.numerator, &other.denominator);
        let bc = Number::mul_vec(&self.denominator, &other.numerator);
        
        // Check if both are zero (special case)
        let self_is_zero = Number::is_zero_vec(&self.numerator);
        let other_is_zero = Number::is_zero_vec(&other.numerator);
        
        if self_is_zero && other_is_zero {
            return true;
        }
        
        // If only one is zero, they're not equal
        if self_is_zero || other_is_zero {
            return false;
        }
        
        // For non-zero numbers, check sign and magnitude
        self.sign == other.sign && Number::cmp_vec(&ad, &bc) == Ordering::Equal
    }
}

impl Eq for Number {}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare a/b with c/d by comparing a*d with b*c
        let self_is_zero = Number::is_zero_vec(&self.numerator);
        let other_is_zero = Number::is_zero_vec(&other.numerator);
        
        // Handle zero cases
        if self_is_zero && other_is_zero {
            return Ordering::Equal;
        }
        if self_is_zero {
            return match other.sign {
                Sign::Positive => Ordering::Less,
                Sign::Negative => Ordering::Greater,
            };
        }
        if other_is_zero {
            return match self.sign {
                Sign::Positive => Ordering::Greater,
                Sign::Negative => Ordering::Less,
            };
        }
        
        // Compare signs first
        match (self.sign, other.sign) {
            (Sign::Positive, Sign::Negative) => Ordering::Greater,
            (Sign::Negative, Sign::Positive) => Ordering::Less,
            (Sign::Positive, Sign::Positive) => {
                // Both positive: compare a*d with b*c
                let ad = Number::mul_vec(&self.numerator, &other.denominator);
                let bc = Number::mul_vec(&self.denominator, &other.numerator);
                Number::cmp_vec(&ad, &bc)
            }
            (Sign::Negative, Sign::Negative) => {
                // Both negative: reverse the comparison
                let ad = Number::mul_vec(&self.numerator, &other.denominator);
                let bc = Number::mul_vec(&self.denominator, &other.numerator);
                Number::cmp_vec(&bc, &ad)
            }
        }
    }
}

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
