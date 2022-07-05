#[derive(Debug, Copy)]
struct N(i32);

impl Clone for N {
    fn clone(&self) -> Self {
        N(self.0)
    }
}

#[test]
fn option_test() {
    let a = N(12);
    //    let a = 12;
    let b = Some(a);
    let a = b.unwrap();

    println!("{}", a.0);
    //    println!("{}",a);
    println!("{:?}", b.unwrap());
}
