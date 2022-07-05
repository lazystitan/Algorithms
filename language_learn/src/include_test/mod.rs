#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let my_string = include!("some.in");
        assert_eq!("🙈🙊🙉🙈🙊🙉", my_string);
        println!("{}", my_string);
        assert!(false);
    }
}
