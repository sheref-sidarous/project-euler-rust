use std::vec::Vec;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, Debug)]
pub struct BigInt {
    internal_rep : Vec<u64>,
}

impl BigInt {
    pub fn new<T>(initial_value : T) -> Self 
    where T : Into<u64>
    {
        BigInt {
            internal_rep : vec!(initial_value.into())
        }
    }

    fn inner_multiply(&self, multiplier : u64) -> BigInt {
        
        let mut acc_result = Vec::new();
        let mut remainder = 0;

        for item in &self.internal_rep {
            //let mut new_carry = false;
            let mul_result = *item as u128 * multiplier as u128;
            let result_lower_half = (mul_result & 0xFFFFFFFFFFFFFFFFu128) as u64;
            let result_upper_half = (mul_result & 0xFFFFFFFFFFFFFFFF0000000000000000u128) as u64;
            let (result, new_carry) = result_lower_half.overflowing_add(remainder);
            remainder = result_upper_half;
            if new_carry {
                remainder += 1;
            }
            acc_result.push(result);
        }
        if remainder > 0 {
            acc_result.push(remainder);
        }

        BigInt { internal_rep: acc_result }
    }
}

impl From<u64> for BigInt {
    fn from(value: u64) -> Self {
        BigInt {
            internal_rep : vec!(value)
        }
    }
}
fn _full_adder(val1 : u64, val2: u64, carry : bool) -> (u64, bool) {
    let  (mut result, mut carry_out) = val1.overflowing_add(val2);
    if carry {
        let mut temp_carry = false;
        (result, temp_carry) = result.overflowing_add(1);
        carry_out |= temp_carry;
    }
    (result, carry_out)
}

impl Add for BigInt {

    type Output = Self;

    fn add(self, other:Self) -> Self {
        let mut result = Vec::new();
        let mut val1_iter = self.internal_rep.iter();
        let mut val2_iter = other.internal_rep.iter();
        let mut carry = false;
        loop {
            let val1 = *val1_iter.next().unwrap_or(&0);
            let val2 = *val2_iter.next().unwrap_or(&0);
            let (result1, carry1) = val1.overflowing_add(if carry {1} else {0});
            let (result2, carry2) = result1.overflowing_add(val2);

            carry = carry1 | carry2;

            if result2 == 0 && !carry{
                break;
            }

            result.push(result2);
        }

        BigInt { internal_rep: result }

    }
}

impl Mul for BigInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result : BigInt = 0.into();
        let mut order = 0;
        for item in rhs.internal_rep {
            let mut x = self.inner_multiply(item);
            for _ in 0..order {
                x.internal_rep.insert(0, 0);
            }
            result = result + x;
            order += 1;
        }

        return result;
    }
}