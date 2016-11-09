extern crate gcc;

fn main() {
    gcc::compile_library("libsendfd.a", &["src/sendfd.c"]);
}
