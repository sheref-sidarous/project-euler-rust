use std::vec::Vec;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, Debug)]
pub struct BigInt {
    internal_rep : Vec<u64>,
}

impl BigInt {

    // consume the string by the decimal step 
    const DECIMAL_STEP : u64 = 3;

    pub fn new<T>(initial_value : T) -> Self 
    where T : Into<u64>
    {
        BigInt {
            internal_rep : vec!(initial_value.into())
        }
    }

    pub fn simple_multiply(&self, multiplier : u64) -> BigInt {
        
        let mut acc_result = Vec::new();
        let mut remainder = 0;

        for item in &self.internal_rep {
            //let mut new_carry = false;
            let mul_result = *item as u128 * multiplier as u128;
            let result_lower_half = (mul_result & 0xFFFFFFFFFFFFFFFFu128) as u64;
            let result_upper_half = ((mul_result & 0xFFFFFFFFFFFFFFFF0000000000000000u128) >> 64) as u64;
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

    pub fn simple_divide(&self, divisor : u64) -> (BigInt, u64) {

        let mut last_op_remainder = 0;
        let mut result_acc = Vec::new();

        for item in self.internal_rep.iter().rev() {            
            let devidend = (*item as u128) | (last_op_remainder as u128) << 64;
            let quotant = devidend / divisor as u128;
            last_op_remainder = devidend % divisor as u128;
            if result_acc.len() != 0 || quotant != 0 {
                result_acc.insert(0, quotant as u64);
            }
        }

        (BigInt { internal_rep: result_acc }, last_op_remainder as u64)
    }

    fn is_zero(&self) -> bool {
        return self.internal_rep.len() == 0 || (self.internal_rep.len() == 1 && self.internal_rep[0] == 0);
    }

    pub fn from_str(input: &str) -> BigInt {

        let mut output : BigInt = 0.into();
        let mut start = 0;
        let mut end = input.len() % Self::DECIMAL_STEP as usize;
        if end == 0 {
            end = Self::DECIMAL_STEP as usize;
        }
        
        loop {
            let chunk_value: u64 = input[start..end].parse().unwrap();
            output = output.add(&chunk_value.into());
            if end == input.len() {
                break;
            } else {
                output = output.simple_multiply(1000u64); // TODO: this should be 10u64^Self::DECIMAL_STEP
            }
            start = end;
            end = end + Self::DECIMAL_STEP as usize;
        };

        output
    }

    pub fn into_str(&self) -> String {
        let mut output = String::new();
        let mut temp = self.clone();
        let mut int_parts = Vec::new();

        while !temp.is_zero() {
            let remainder;
            (temp, remainder) = temp.simple_divide(1000u64); // TODO: this should be 10u64^Self::DECIMAL_STEP
            int_parts.push(remainder);
        }

        for part in int_parts.iter().rev() {
            output.push_str(format!("{:03}", *part).as_str());
        }

        output
    }
}

impl From<u64> for BigInt {
    fn from(value: u64) -> Self {
        BigInt {
            internal_rep : vec!(value)
        }
    }
}

fn full_adder(val1 : u64, val2: u64, carry : bool) -> (u64, bool) {
    let  (mut result, mut carry_out) = val1.overflowing_add(val2);
    if carry {
        let temp_carry;
        (result, temp_carry) = result.overflowing_add(1);
        carry_out |= temp_carry;
    }
    (result, carry_out)
}

impl Add for &BigInt {

    type Output = BigInt;

    fn add(self, other:Self) -> BigInt {
        let mut acc_result = Vec::new();
        let mut val1_iter = self.internal_rep.iter();
        let mut val2_iter = other.internal_rep.iter();
        let mut carry = false;
        loop {
            let val1 = *val1_iter.next().unwrap_or(&0);
            let val2 = *val2_iter.next().unwrap_or(&0);
            let result : u64;
            (result, carry) = full_adder(val1, val2, carry);

            if result == 0 && !carry{
                break;
            }

            acc_result.push(result);
        }

        BigInt { internal_rep: acc_result }

    }
}

impl Mul for &BigInt {
    type Output = BigInt;

    fn mul(self, rhs: Self) -> BigInt {
        let mut result : BigInt = 0.into();
        let mut order = 0;
        for item in &rhs.internal_rep {
            let mut x = self.simple_multiply(*item);
            for _ in 0..order {
                x.internal_rep.insert(0, 0);
            }
            result = &result + &x;
            order += 1;
        }

        return result;
    }
}

/* TODO: Division, some promising resources :
https://ridiculousfish.com/blog/posts/labor-of-division-episode-iv.html
https://surface.syr.edu/cgi/viewcontent.cgi?article=1162&context=eecs_techreports
https://skanthak.homepage.t-online.de/division.html
https://en.wikipedia.org/wiki/Division_algorithm

 */