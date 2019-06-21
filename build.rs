extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/c_files/double.c")
        .compile("double");
}