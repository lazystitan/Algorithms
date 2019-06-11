mod string_match;

use string_match::{Search, Kmp, RabinKarp, Bm};

fn main() {
    let text = "where am i? what the fuck is this?".to_string();
    let pattern = "what".to_string();

    let rk = RabinKarp::new(pattern);
    let position = rk.search(&text);
}
