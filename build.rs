fn main() {
    cc::Build::new()
        .file("src/c/inc.c")
        .flag("-ffast-math")
        .compile("inc");
}
