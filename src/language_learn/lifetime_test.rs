#[cfg(test)]
mod test {

//    fn as_str<'a>(data : &'a u32) -> &'a str {
//        let s = format!("{}",data);
//        &s
//    }
    #[test]
    fn main_test() {
        let mut data = vec![1,2,3];
        let mut x = &data[0];
        println!("{}",x);
        data.push(4);
        x = &data[3];
        println!("{}",x);
    }
}