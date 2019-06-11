#[macro_use]
mod lib;
mod string_match;

use string_match::{Bm, Kmp, RabinKarp, Search};
use Algorithms::for_test::Line;


fn main() {
    let mut line = Line(0);
    for i in line {
        println!("{}", i);
    }

    several_times!(1,2,3);

}
