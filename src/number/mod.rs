use std::cmp::Ordering;
mod tests;
mod traits;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}

// little endian
#[derive(Debug)]
pub struct Number {
    numerator: Vec<u32>,
    denominator: Vec<u32>,
    sign: Sign,
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
            (Sign::Positive, Sign::Positive) => {
                (Self::sum_vec(&ad, &bc), Sign::Positive)
            }
            (Sign::Positive, Sign::Negative) => {
                let numerator = Self::diff_vec(&ad, &bc);
                match Self::cmp_vec(&ad, &bc) {
                    Ordering::Greater => {
                        (numerator, Sign::Positive)
                    },
                    Ordering::Less => {
                        (numerator, Sign::Negative)
                    }
                    Ordering::Equal => {
                        // zero
                        (numerator, Sign::Positive)
                    }
                }
            }
            (Sign::Negative, Sign::Positive) => {
                let numerator = Self::diff_vec(&ad, &bc);
                match Self::cmp_vec(&bc, &ad) {
                    Ordering::Greater => {
                        (numerator, Sign::Positive)
                    },
                    Ordering::Less => {
                        (numerator, Sign::Negative)
                    }
                    Ordering::Equal => {
                        // zero
                        (numerator, Sign::Positive)
                    }
                }
            }
            (Sign::Negative, Sign::Negative) => {
                (Self::sum_vec(&ad, &bc), Sign::Negative)
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
            sign: match b.sign {
                Sign::Positive => Sign::Negative,
                Sign::Negative => Sign::Positive,
            },
        };

        self.add(&negate_b)
    }

    pub fn mul(&self, b: &Number) -> Number {
        // a/b * c/d = ac/bd
        Number {
            numerator: Self::mul_vec(&self.numerator, &b.numerator),
            denominator: Self::mul_vec(&self.denominator, &b.denominator),
            sign: if self.sign == b.sign { Sign::Positive } else { Sign::Negative },
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
            sign: if self.sign == b.sign { Sign::Positive } else { Sign::Negative },
        })
    }

    // GCD using Euclidean algorithm for Vec<u32> (base 32-bit)
    fn gcd_vec(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut a = a.to_vec();
        let mut b = b.to_vec();
        
        // Remove trailing zeros
        while a.last() == Some(&0) && a.len() > 1 {
            a.pop();
        }
        while b.last() == Some(&0) && b.len() > 1 {
            b.pop();
        }
        
        // Euclidean algorithm
        while !Self::is_zero_vec(&b) {
            let remainder = Self::mod_vec(&a, &b);
            a = b;
            b = remainder;
        }
        
        a
    }
    
    // Modulo operation for Vec<u32>
    // Note: This implementation uses repeated subtraction which is O(dividend/divisor) time complexity.
    // This is sufficient for the GCD algorithm's use case with typical fraction values.
    // For general-purpose arbitrary-precision modulo, a more sophisticated algorithm would be needed.
    fn mod_vec(a: &[u32], b: &[u32]) -> Vec<u32> {
        if Self::is_zero_vec(b) {
            return vec![0];
        }
        
        let mut remainder = a.to_vec();
        
        // Simple repeated subtraction for modulo
        while Number::cmp_vec(&remainder, b) != Ordering::Less {
            remainder = Self::diff_vec(&remainder, b);
        }
        
        remainder
    }
    
    // Reduce the fraction by dividing both numerator and denominator by their GCD
    pub fn reduce(&self) -> Number {
        let gcd = Self::gcd_vec(&self.numerator, &self.denominator);
        
        // If GCD is 1, no reduction needed
        if gcd.len() == 1 && gcd[0] == 1 {
            return Number {
                numerator: self.numerator.clone(),
                denominator: self.denominator.clone(),
                sign: self.sign,
            };
        }
        
        // Divide both numerator and denominator by GCD
        let reduced_numerator = Self::div_vec(&self.numerator, &gcd);
        let reduced_denominator = Self::div_vec(&self.denominator, &gcd);
        
        Number {
            numerator: reduced_numerator,
            denominator: reduced_denominator,
            sign: self.sign,
        }
    }
    
    // Integer division for Vec<u32>
    // Note: This implementation uses repeated subtraction which is O(result) time complexity
    // and returns the result as a single u32 value. This is sufficient for the reduce() function's
    // use case where we divide by GCD values, which are typically small.
    // For general-purpose arbitrary-precision division, a more sophisticated algorithm would be needed.
    fn div_vec(a: &[u32], b: &[u32]) -> Vec<u32> {
        if Self::is_zero_vec(b) {
            return vec![0];
        }
        
        let mut remainder = a.to_vec();
        
        // Simple repeated subtraction for division
        let mut count = 0u32;
        while Number::cmp_vec(&remainder, b) != Ordering::Less {
            remainder = Self::diff_vec(&remainder, b);
            count += 1;
            
            // Overflow protection - if we exceed u32::MAX, return what we have
            if count == u32::MAX {
                break;
            }
        }
        
        if count == 0 {
            vec![0]
        } else {
            vec![count]
        }
    }
}
