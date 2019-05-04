extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/c_stub.c")
        .compile("libcstub.a");
}
