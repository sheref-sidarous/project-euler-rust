
static mut no_of_paths : u64 = 0;
static mut no_of_calls : u64 = 0;

fn visit<const G : u32>(x : u32, y: u32) {

    unsafe { no_of_calls += 1 };

    if x == G && y == G {
        unsafe { no_of_paths += 1 };
    }

    if x < G {
        visit::<G>(x +1, y);
    }

    if y < G {
        visit::<G>(x, y+1);
    }

}

pub fn solution() {

    visit::<20>(0, 0);
    unsafe { println!("{}", no_of_paths) };
    unsafe { println!("{}", no_of_calls) };

}