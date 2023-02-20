extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/add.cpp")
        .cpp(true)
        .compile("libadd.a");
}
