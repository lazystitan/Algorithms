#[derive(Debug)]
struct N(i32);

#[test]
fn option_test() {
//    let a = N(12); error
    let a = 12;
    let b = Some(a);
    let a = b.unwrap();

//    println!("{}", a.0);
    println!("{}",a);
    println!("{:?}", b.unwrap());
}