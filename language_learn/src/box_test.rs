#[derive(Debug)]
struct Config {
    a: String,
    b: String
}

static mut CONFIG: Option<&mut Config> = None;

pub fn main() {
    let c = Box::new(Config{
        a: "a".to_string(),
        b: "b".to_string()
    });
    unsafe {
        CONFIG = Some(Box::leak(c));
        println!("{:?}", CONFIG);
    }
}
