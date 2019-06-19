mod string_match;
mod math;
mod basic_type;
mod sort;
mod thread_pool;
//mod no_mod_test;

use rand::prelude::*;
use sort::{SelectSort, InsertSort, ShellSort, SortTrait};
use std::thread;
use thread_pool::ThreadPool;
use std::time::Duration;

fn main() {
    let pool = thread_pool::ThreadPool::new(4);
    pool.execute(|| {
        thread::sleep(Duration::from_secs(3));
        println!("first");
    });

    pool.execute(|| {
        thread::sleep(Duration::from_secs(2));
        println!("second");
    });

    pool.execute(|| {
        thread::sleep(Duration::from_secs(1));
        println!("third");
    });

}