
pub fn solution() {

  let mut m = 1;
  let mut count = 0;

  
  'outer: loop {
    let a = m;
    for b in 1..a+1 {
      for c in 1..b+1 {

        let a_f64 = a as f64;
        let b_f64 = b as f64;
        let c_f64 = c as f64;

        let len = (a_f64 * a_f64 + b_f64*b_f64 + c_f64*c_f64 + 2_f64*b_f64*c_f64).sqrt();

        if len == len.ceil() {
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
}