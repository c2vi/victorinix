
fn main() {
    cc::Build::new()
        .file("src/main.c")
        .flag("-w")
        .compile("victorinix");
    println!("cargo:rerun-if-changed=src/main.c");
}

