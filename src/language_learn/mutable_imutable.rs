
fn immutable_fn(s : &String) {
    println!("immutable fn");
}

fn mutable_fn(s : &mut String) {
    println!("mutable fn");
}


#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test() {
/// cannot use a mutable borrow when exist an immutable borrow.
/// error code eg1:
///```
///        let mut a = String::from("test");
///        let c = &mut a;
///        immutable_fn(c);
///        {
///            let b = &a;
///            println!("{:?}", b.as_bytes());
///        }
///        immutable_fn(c);
/// ```
/// error code eg2:
/// ```
///        let mut a = String::from("test");
///        let c = &mut a;
///        {
///            let b = &a;
///            println!("{:?}", b.as_bytes());
///        }
///        immutable_fn(c);
///```
        let a = 32;
    }
    //result : a mutable borrow can transfer as an immutable reference parameter
}