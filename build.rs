fn main() {
    println!("cargo:rustc-link-lib=dylib=pulse");
    println!("cargo:rustc-link-lib=dylib=pulse-simple");
    println!("cargo:rustc-link-lib=dylib=asound");
}