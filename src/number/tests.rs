#![cfg(test)]
use super::{Number, Sign};

fn from_i32(val: i32) -> Number {
    Number {
        numerator: vec![val.unsigned_abs()],
        denominator: vec![1],
        sign: if val >= 0 { Sign::Positive } else { Sign::Negative },
    }
}

fn from_fraction(n: u32, d: u32, sign: Sign) -> Number {
    Number {
        numerator: vec![n],
        denominator: vec![d],
        sign,
    }
}

#[test]
fn test_add_positive() {
    let a = from_i32(2);
    let b = from_i32(3);
    let c = a.add(&b);
    assert_eq!(c.numerator, vec![5]);
    assert_eq!(c.denominator, vec![1]);
    assert_eq!(c.sign, Sign::Positive);
}

#[test]
fn test_add_negative() {
    let a = from_i32(-2);
    let b = from_i32(-3);
    let c = a.add(&b);
    assert_eq!(c.numerator, vec![5]);
    assert_eq!(c.denominator, vec![1]);
    assert_eq!(c.sign, Sign::Negative);
}

#[test]
fn test_add_opposite_signs() {
    let a = from_i32(5);
    let b = from_i32(-3);
    let c = a.add(&b);
    assert_eq!(c.numerator, vec![2]);
    assert_eq!(c.denominator, vec![1]);
    assert_eq!(c.sign, Sign::Positive);

    let d = from_i32(-5);
    let e = from_i32(3);
    let f = d.add(&e);
    assert_eq!(f.numerator, vec![2]);
    assert_eq!(f.denominator, vec![1]);
    assert_eq!(f.sign, Sign::Negative);
}

#[test]
fn test_sub() {
    let a = from_i32(10);
    let b = from_i32(7);
    let c = a.sub(&b);
    assert_eq!(c.numerator, vec![3]);
    assert_eq!(c.denominator, vec![1]);
    assert_eq!(c.sign, Sign::Positive);

    let d = from_i32(7);
    let e = from_i32(10);
    let f = d.sub(&e);
    assert_eq!(f.numerator, vec![3]);
    assert_eq!(f.denominator, vec![1]);
    assert_eq!(f.sign, Sign::Negative);
}

#[test]
fn test_sub_opposite_signs() {
    let a = from_i32(5);
    let b = from_i32(-3);
    let c = a.sub(&b);
    assert_eq!(c.numerator, vec![8]);
    assert_eq!(c.denominator, vec![1]);
    assert_eq!(c.sign, Sign::Positive);

    let d = from_i32(-5);
    let e = from_i32(3);
    let f = d.sub(&e);
    assert_eq!(f.numerator, vec![8]);
    assert_eq!(f.denominator, vec![1]);
    assert_eq!(f.sign, Sign::Negative);
}


#[test]
fn test_mul() {
    let a = from_i32(4);
    let b = from_i32(5);
    let c = a.mul(&b);
    assert_eq!(c.numerator, vec![20]);
    assert_eq!(c.denominator, vec![1]);
    assert_eq!(c.sign, Sign::Positive);

    let d = from_i32(-4);
    let e = from_i32(5);
    let f = d.mul(&e);
    assert_eq!(f.numerator, vec![20]);
    assert_eq!(f.denominator, vec![1]);
    assert_eq!(f.sign, Sign::Negative);
}

#[test]
fn test_div() {
    let a = from_i32(10);
    let b = from_i32(2);
    let c = a.div(&b).unwrap();
    assert_eq!(c.numerator, vec![10]);
    assert_eq!(c.denominator, vec![2]);
    assert_eq!(c.sign, Sign::Positive);

    let d = from_i32(-10);
    let e = from_i32(2);
    let f = d.div(&e).unwrap();
    assert_eq!(f.numerator, vec![10]);
    assert_eq!(f.denominator, vec![2]);
    assert_eq!(f.sign, Sign::Negative);
}

#[test]
fn test_fractional_addition() {
    let a = from_fraction(1, 2, Sign::Positive); // 1/2
    let b = from_fraction(1, 3, Sign::Positive); // 1/3
    let c = a.add(&b);
    // 1/2 + 1/3 = (1*3 + 2*1)/6 = (3 + 2)/6 = 5/6
    assert_eq!(c.numerator, vec![5]);
    assert_eq!(c.denominator, vec![6]);
    assert_eq!(c.sign, Sign::Positive);
}

#[test]
fn test_zero_addition() {
    let zero = from_i32(0);
    let a = from_i32(123);
    let c = a.add(&zero);
    assert_eq!(c.numerator, vec![123]);
    assert_eq!(c.denominator, vec![1]);
    assert_eq!(c.sign, Sign::Positive);
}

#[test]
fn test_divide_by_zero() {
    let a = from_i32(10);
    let zero = from_i32(0);
    assert!(a.div(&zero).is_none());
}

#[test]
fn test_partial_eq() {
    let a = from_i32(5);
    let b = from_i32(5);
    assert_eq!(a, b);
    
    let c = from_i32(-5);
    let d = from_i32(-5);
    assert_eq!(c, d);
    
    let e = from_i32(3);
    let f = from_i32(4);
    assert_ne!(e, f);
    
    // Test fraction equality
    let g = from_fraction(1, 2, Sign::Positive);
    let h = from_fraction(2, 4, Sign::Positive);
    assert_eq!(g, h); // 1/2 == 2/4
}

#[test]
fn test_zero_equality() {
    let zero1 = from_i32(0);
    let zero2 = from_fraction(0, 1, Sign::Positive);
    let zero3 = from_fraction(0, 5, Sign::Negative);
    assert_eq!(zero1, zero2);
    assert_eq!(zero1, zero3);
}

#[test]
fn test_partial_ord() {
    let a = from_i32(5);
    let b = from_i32(3);
    assert!(a > b);
    assert!(b < a);
    
    let c = from_i32(-5);
    let d = from_i32(-3);
    assert!(c < d); // -5 < -3
    
    let e = from_i32(2);
    let f = from_i32(-2);
    assert!(e > f);
}

#[test]
fn test_ord_with_fractions() {
    let a = from_fraction(1, 2, Sign::Positive); // 1/2
    let b = from_fraction(1, 3, Sign::Positive); // 1/3
    assert!(a > b);
    
    let c = from_fraction(2, 3, Sign::Positive); // 2/3
    let d = from_fraction(3, 4, Sign::Positive); // 3/4
    assert!(d > c);
}

#[test]
fn test_ord_with_zero() {
    let zero = from_i32(0);
    let positive = from_i32(5);
    let negative = from_i32(-5);
    
    assert!(positive > zero);
    assert!(zero > negative);
    assert!(positive > negative);
}

#[test]
fn test_reduce() {
    // 2/4 should reduce to 1/2
    let a = from_fraction(2, 4, Sign::Positive);
    let reduced = a.reduce();
    assert_eq!(reduced.numerator, vec![1]);
    assert_eq!(reduced.denominator, vec![2]);
    
    // 6/9 should reduce to 2/3
    let b = from_fraction(6, 9, Sign::Positive);
    let reduced_b = b.reduce();
    assert_eq!(reduced_b.numerator, vec![2]);
    assert_eq!(reduced_b.denominator, vec![3]);
    
    // 5/7 is already reduced
    let c = from_fraction(5, 7, Sign::Positive);
    let reduced_c = c.reduce();
    assert_eq!(reduced_c.numerator, vec![5]);
    assert_eq!(reduced_c.denominator, vec![7]);
}

#[test]
fn test_reduce_with_sign() {
    let a = from_fraction(4, 6, Sign::Negative);
    let reduced = a.reduce();
    assert_eq!(reduced.numerator, vec![2]);
    assert_eq!(reduced.denominator, vec![3]);
    assert_eq!(reduced.sign, Sign::Negative);
}

#[test]
fn test_reduce_large_numbers() {
    let a = from_fraction(100, 150, Sign::Positive);
    let reduced = a.reduce();
    assert_eq!(reduced.numerator, vec![2]);
    assert_eq!(reduced.denominator, vec![3]);
}
