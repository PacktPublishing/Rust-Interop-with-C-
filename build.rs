extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/add.cpp")
        .compile("libadd.a");
}
