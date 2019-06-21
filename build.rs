//created for learn cc

extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/language_learn/c_files/double.c")
        .compile("double");
}