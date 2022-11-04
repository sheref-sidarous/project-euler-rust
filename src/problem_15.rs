use std::future::Future;
use std::future::join;
use async_recursion::async_recursion;

static mut no_of_paths : u64 = 0;
static mut no_of_calls : u64 = 0;

#[async_recursion]
async fn visit<const G : u32>(x : u32, y: u32) {

    unsafe { no_of_calls += 1 };

    if x == G && y == G {
        unsafe { no_of_paths += 1 };
    }

    let x_branch = if x < G {
        Some(visit::<G>(x +1, y))
    } else {
        None
    };

    let y_branch = if y < G {
        Some(visit::<G>(x, y+1))
    } else {
        None
    };

    match (x_branch, y_branch) {
        (Some(x), Some(y)) => {join!(x,y).await;},
        (Some(x), None) => x.await,
        (None, Some(y)) => y.await,
        (None, None) => {},
    };


}

pub async fn solution() {

    visit::<20>(0, 0).await;
    unsafe { println!("{}", no_of_paths) };
    unsafe { println!("{}", no_of_calls) };

}