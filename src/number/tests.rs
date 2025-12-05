#![cfg(test)]
use super::Number;

fn from_i32(val: i32) -> Number {
    Number {
        numerator: vec![val.unsigned_abs()],
        denominator: vec![1],
        sign: val >= 0,
    }
}

fn from_fraction(n: u32, d: u32, sign: bool) -> Number {
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
    assert!(c.sign)
}

#[test]
fn test_add_negative() {
    let a = from_i32(-2);
    let b = from_i32(-3);
    let c = a.add(&b);
    assert_eq!(c.numerator, vec![5]);
    assert_eq!(c.denominator, vec![1]);
    assert!(!c.sign);
}

#[test]
fn test_add_opposite_signs() {
    let a = from_i32(5);
    let b = from_i32(-3);
    let c = a.add(&b);
    assert_eq!(c.numerator, vec![2]);
    assert_eq!(c.denominator, vec![1]);
    assert!(c.sign);

    let d = from_i32(-5);
    let e = from_i32(3);
    let f = d.add(&e);
    assert_eq!(f.numerator, vec![2]);
    assert_eq!(f.denominator, vec![1]);
    assert!(!f.sign);
}

#[test]
fn test_sub() {
    let a = from_i32(10);
    let b = from_i32(7);
    let c = a.sub(&b);
    assert_eq!(c.numerator, vec![3]);
    assert_eq!(c.denominator, vec![1]);
    assert!(c.sign);

    let d = from_i32(7);
    let e = from_i32(10);
    let f = d.sub(&e);
    assert_eq!(f.numerator, vec![3]);
    assert_eq!(f.denominator, vec![1]);
    assert!(!f.sign);
}

#[test]
fn test_sub_opposite_signs() {
    let a = from_i32(5);
    let b = from_i32(-3);
    let c = a.sub(&b);
    assert_eq!(c.numerator, vec![8]);
    assert_eq!(c.denominator, vec![1]);
    assert!(c.sign);

    let d = from_i32(-5);
    let e = from_i32(3);
    let f = d.sub(&e);
    assert_eq!(f.numerator, vec![8]);
    assert_eq!(f.denominator, vec![1]);
    assert!(!f.sign);
}


#[test]
fn test_mul() {
    let a = from_i32(4);
    let b = from_i32(5);
    let c = a.mul(&b);
    assert_eq!(c.numerator, vec![20]);
    assert_eq!(c.denominator, vec![1]);
    assert!(c.sign);

    let d = from_i32(-4);
    let e = from_i32(5);
    let f = d.mul(&e);
    assert_eq!(f.numerator, vec![20]);
    assert_eq!(f.denominator, vec![1]);
    assert!(!f.sign);
}

#[test]
fn test_div() {
    let a = from_i32(10);
    let b = from_i32(2);
    let c = a.div(&b).unwrap();
    assert_eq!(c.numerator, vec![10]);
    assert_eq!(c.denominator, vec![2]);
    assert!(c.sign);

    let d = from_i32(-10);
    let e = from_i32(2);
    let f = d.div(&e).unwrap();
    assert_eq!(f.numerator, vec![10]);
    assert_eq!(f.denominator, vec![2]);
    assert!(!f.sign);
}

#[test]
fn test_fractional_addition() {
    let a = from_fraction(1, 2, true); // 1/2
    let b = from_fraction(1, 3, true); // 1/3
    let c = a.add(&b);
    // 1/2 + 1/3 = (1*3 + 2*1)/6 = (3 + 2)/6 = 5/6
    assert_eq!(c.numerator, vec![5]);
    assert_eq!(c.denominator, vec![6]);
    assert!(c.sign);
}

#[test]
fn test_zero_addition() {
    let zero = from_i32(0);
    let a = from_i32(123);
    let c = a.add(&zero);
    assert_eq!(c.numerator, vec![123]);
    assert_eq!(c.denominator, vec![1]);
    assert!(c.sign);
}

#[test]
fn test_divide_by_zero() {
    let a = from_i32(10);
    let zero = from_i32(0);
    assert!(a.div(&zero).is_none());
}
