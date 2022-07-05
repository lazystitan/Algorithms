mod cc_rs_libc_test;
mod include_test;
#[macro_use]
mod macro_test;
mod compare_option;
mod data_representation;
mod lifetime_test;
mod map_bug;
mod mutable_immutable;
mod option_test;

mod language_learn_cc_rs_libc_test_lib {
    use crate::cc_rs_libc_test::{another_function, show_some_content};

    pub fn call(input: i32) -> i32 {
        unsafe { another_function(input) as i32 }
    }

    pub fn call_show() {
        unsafe {
            show_some_content();
        }
    }
}

mod test_in_mod {
    #[test]
    fn some() {
        println!("some");
        assert!(false);
    }

    #[test]
    fn test() {
        macro_expr!(1, 2, 3);
        assert!(false);
    }
}

use language_learn_cc_rs_libc_test_lib::{call, call_show};

fn main() {
    call_show();
    println!("{}", call(12) as i32);
    println!("Hello, world!");
}
