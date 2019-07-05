#[derive(PartialOrd, PartialEq)]
struct Integer(i32);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let a = Some(Integer(40));
        let b = Some(Integer(50));
        assert_eq!(a > b, 40 > 50);

///returns false when compared None with Some(_)
        let c = None;
        assert!(!(c > b));

        let a = Some(1);
        let b = Some(2);
        assert_eq!(a < b, 1 < 2);
    }
}