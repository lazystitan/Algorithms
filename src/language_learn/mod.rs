pub mod include_test;
pub mod cc_rs_libc_test;
#[macro_use]
mod macro_test;
mod mutable_immutable;
mod compare_option;
mod option_test;
mod map_bug;
mod data_representation;
mod lifetime_test;

mod test_in_mod{
    #[test]
    fn some() {
        println!("some");
        assert!(false);
    }

    #[test]
    fn test() {
        macro_expr!(1,2,3);
        assert!(false);
    }
}