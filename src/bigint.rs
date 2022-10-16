use std::vec::Vec;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug)]
pub struct BigInt {
    internal_rep : Vec<i64>,
}

impl BigInt {
    pub fn new<T>(initial_value : T) -> Self 
    where T : Into<i64>
    {
        BigInt {
            internal_rep : vec!(initial_value.into())
        }
    }
}

impl From<i64> for BigInt {
    fn from(value: i64) -> Self {
        BigInt {
            internal_rep : vec!(value)
        }
    }
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
