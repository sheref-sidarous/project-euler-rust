use std::collections::HashMap;
use once_cell::sync::Lazy;

fn triangle_number_core(order : u32) -> u64 {
    if order == 1 {
        1
    } else {
        order as u64 + triangle_number(order - 1)
    }
}


fn triangle_number(order : u32) -> u64 {

    static mut FOUND_TRIANGLE_NUMBERS : Lazy<HashMap<u32, u64>> = Lazy::new(|| { HashMap::new() });

    let result = match unsafe { FOUND_TRIANGLE_NUMBERS.get(&order) } {
        Some(value) => *value,
        None => {
            let new_result = triangle_number_core(order);
            unsafe { FOUND_TRIANGLE_NUMBERS.insert(order, new_result) };
            new_result
        }

    };

    result
}

pub fn solution() {
    println!("{}", triangle_number(7))
}