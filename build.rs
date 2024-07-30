use std::env;

fn main() {
    if env::var("PROFILE").unwrap() == "release" {
        println!("cargo:rustc-cfg=build_release");
    }
}
