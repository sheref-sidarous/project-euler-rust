#![feature(future_join)]

mod bigint;
mod indexed_iter;

//mod problem_8;
//mod problem_12;
//mod problem_14;
mod problem_15;

use tokio;
use std::future::join;

fn main() {
    
    let mut rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .build()
        .unwrap();

    //let task1 = rt.spawn(problem_15::solution());
    //let task2 = rt.spawn(problem_15::solution());

    rt.block_on( async {
        //join!(task1, task2).await;
        join!(problem_15::solution(), problem_15::solution()).await;
    } );
}
