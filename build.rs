use std::env;

fn main() {
    if env::var("CARGO_CFG_TARGET_VENDOR").unwrap() == "apple" {
        println!("cargo:rustc-cdylib-link-arg=-install_name");
        println!("cargo:rustc-cdylib-link-arg=@rpath/libsignal_ffi.dylib");
    }
}
