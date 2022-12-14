
static mut is_a_square_number_entry_cnt : u64 = 0;
static mut is_a_square_number_entry_cnt_opt : u64 = 0;

fn is_a_square_number(input : u64) -> bool {

  unsafe {
  is_a_square_number_entry_cnt += 1;
  }

  let last_digit = input / 10;

  match last_digit {
    2 | 3 | 7 | 8 => {
      unsafe {is_a_square_number_entry_cnt_opt += 1; }
      // can't be a square number
      return false;
    },

    _ => {}
  }

  let sq_root = (input as f64).sqrt();
  return  sq_root == sq_root.ceil();

}

pub fn solution() {

  let mut m = 1_u64;
  let mut count = 0;

  
  'outer: loop {
    let a = m;
    for b in 1..a+1 {
      for c in 1..b+1 {

        let a_f64 = a as u64;
        let b_f64 = b as u64;
        let c_f64 = c as u64;

        let len_sq = a_f64 * a_f64 + b_f64*b_f64 + c_f64*c_f64 + 2_u64*b_f64*c_f64;

        if is_a_square_number(len_sq) {
          count += 1;
        }
      }
    }
    if count >= 1000000 {
      break 'outer;
    }
    m += 1;
  }

  println!("cnt={}, M={}", count, m);

  unsafe {
    println!("is_a_square_number_entry_cnt={}, is_a_square_number_entry_cnt_opt={}", is_a_square_number_entry_cnt, is_a_square_number_entry_cnt_opt);
  }
}