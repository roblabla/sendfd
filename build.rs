extern crate cc;

fn main() {
    cc::Build::new().file("src/sendfd.c").compile("libsendfd.a");
}
