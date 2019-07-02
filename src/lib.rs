mod language_learn;

pub mod language_learn_cc_rs_libc_test_lib{
    use super::language_learn::cc_rs_libc_test::{ another_function, show_some_content };
    pub fn call(input: i32) -> i32 {
        unsafe {
            another_function(input) as i32
        }
    }

    pub fn call_show() {
        unsafe {
            show_some_content();
        }
    }
}