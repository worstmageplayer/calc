use std::cmp::Ordering;
mod tests;
mod traits;

// little endian
pub struct Number {
    numerator: Vec<u32>,
    denominator: Vec<u32>,
    sign: bool,
}

impl Number {
    fn sum_vec(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut result = Vec::new();
        let mut carry = 0u64;
        let max_len = a.len().max(b.len());

        for i in 0..max_len  {
            let a_val = match a.get(i) {
                Some(a) => *a,
                None => 0u32,
            };
            let b_val = match b.get(i) {
                Some(a) => *a,
                None => 0u32,
            };
            let sum = a_val as u64 + b_val as u64 + carry;
            result.push(sum as u32);
            carry = sum >> 32;
        }

        if carry > 0 {
            result.push(carry as u32);
        }

        result
    }

    fn diff_vec(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();
        let mut borrow = 0i64;

        match Self::cmp_vec(a, b) {
            Ordering::Greater => {
                for (i, &a_elem) in a.iter().enumerate() {
                    let a_val = a_elem as i64;
                    let b_val = match b.get(i) {
                        Some(b) => *b as i64,
                        None => 0i64,
                    };
                    let diff = a_val - b_val - borrow;

                    if diff < 0 {
                        result.push((diff + (1i64 << 32)) as u32);
                        borrow = 1;
                    } else {
                        result.push(diff as u32);
                        borrow = 0;
                    }
                }
            }
            Ordering::Less => {
                for (i, &a_elem) in b.iter().enumerate() {
                    let a_val = a_elem as i64;
                    let b_val = match a.get(i) {
                        Some(b) => *b as i64,
                        None => 0i64,
                    };
                    let diff = a_val - b_val - borrow;

                    if diff < 0 {
                        result.push((diff + (1i64 << 32)) as u32);
                        borrow = 1;
                    } else {
                        result.push(diff as u32);
                        borrow = 0;
                    }

                }
            }
            Ordering::Equal => {
                return vec![0u32];
            }
        }

        while result.last() == Some(&0) && result.len() > 1 {
            result.pop();
        }
        result

    }

    fn mul_vec(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut result = vec![0u32; a.len() + b.len()];

        for i in 0..a.len() {
            let mut carry = 0u64;
            for j in 0..b.len() {
                let ab = (a[i] as u64) * (b[j] as u64) + result[i + j] as u64 + carry;
                result[i + j] = ab as u32;
                carry = ab >> 32;
            }
            result[i + b.len()] = carry as u32;
        }

        while result.last() == Some(&0) && result.len() > 1 {
            result.pop();
        }
        result

    }

    fn cmp_vec(a: &[u32], b: &[u32]) -> Ordering {

        if a.len() != b.len() {
            return a.len().cmp(&b.len());
        }

        for i in (0..a.len()).rev() {
            match a[i].cmp(&b[i]) {
                Ordering::Equal => continue,
                other => return other,
            }
        }

        Ordering::Equal
    }

    pub fn add(&self, b: &Number) -> Number {
        // a/b + c/d = ad + bc / cd
        let ad = Self::mul_vec(&self.numerator, &b.denominator);
        let bc = Self::mul_vec(&self.denominator, &b.numerator);
        let bd = Self::mul_vec(&self.denominator, &b.denominator);

        let (numerator, sign) = match (self.sign, b.sign) {
            (true, true) => {
                (Self::sum_vec(&ad, &bc), true)
            }
            (true, false) => {
                let numerator = Self::diff_vec(&ad, &bc);
                match Self::cmp_vec(&ad, &bc) {
                    Ordering::Greater => {
                        (numerator, true)
                    },
                    Ordering::Less => {
                        (numerator, false)
                    }
                    Ordering::Equal => {
                        // zero
                        (numerator, true)
                    }
                }
            }
            (false, true) => {
                let numerator = Self::diff_vec(&ad, &bc);
                match Self::cmp_vec(&bc, &ad) {
                    Ordering::Greater => {
                        (numerator, true)
                    },
                    Ordering::Less => {
                        (numerator, false)
                    }
                    Ordering::Equal => {
                        // zero
                        (numerator, true)
                    }
                }
            }
            (false, false) => {
                (Self::sum_vec(&ad, &bc), false)
            }
        };
        Number {
            numerator,
            denominator: bd,
            sign,
        }
    }

    pub fn sub(&self, b: &Number) -> Number {
        let negate_b = Number {
            numerator: b.numerator.clone(),
            denominator: b.denominator.clone(),
            sign: !b.sign,
        };

        self.add(&negate_b)
    }

    pub fn mul(&self, b: &Number) -> Number {
        // a/b * c/d = ac/bd
        Number {
            numerator: Self::mul_vec(&self.numerator, &b.numerator),
            denominator: Self::mul_vec(&self.denominator, &b.denominator),
            sign: self.sign == b.sign,
        }
    }

    fn is_zero_vec(v: &[u32]) -> bool {
        v.iter().all(|&x| x == 0)
    }

    pub fn div(&self, b: &Number) -> Option<Number> {
        if Self::is_zero_vec(&b.numerator) {
            return None;
        }

        Some(Number {
            numerator: Self::mul_vec(&self.numerator, &b.denominator),
            denominator: Self::mul_vec(&self.denominator, &b.numerator),
            sign: self.sign == b.sign,
        })
    }
}
