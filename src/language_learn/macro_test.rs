//#[macro_export]
//#[macro_use]
macro_rules! macro_expr {
    ($($a : expr),+) => {
        {$(println!("{}",$a);)+}
    };
}

#[cfg(test)]
mod test {
//    use super::*;

    #[test]
    fn test() {
        macro_expr!(1,2,3);
        assert!(false);
    }

    #[test]
    fn some() {
        println!("some");
        assert!(false);
    }
}