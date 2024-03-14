
fn main() {
    cc::Build::new()
        .file("src/main.c")
        .compile("victorinix");
    println!("cargo:rerun-if-changed=src/main.c");
}

