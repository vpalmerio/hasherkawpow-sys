use cmake::{self, Config};

fn main() {
    let dst = Config::new("src")
        .build_target("hasherkawpow")
        .very_verbose(true)
        .always_configure(false)
        .build();

    let toolchain_environment = std::env::var("CARGO_CFG_TARGET_ENV").unwrap();

    if toolchain_environment == "gnu" {
        println!("cargo:rustc-link-lib=stdc++"); // link to the C++ standard library
        println!("cargo:rustc-link-search=all={}/build", dst.display());
        println!("cargo:rustc-link-lib=static=hasherkawpow");
    } else if toolchain_environment == "msvc" {
        let profile = std::env::var("PROFILE").unwrap_or("debug".to_string());
        let config = if profile == "release" {
            "Release"
        } else {
            "Debug"
        };
        println!(
            "cargo:rustc-link-search=all={}/build/{}",
            dst.display(),
            config
        );
        println!("cargo:rustc-link-lib=static=hasherkawpow");
    } else if toolchain_environment.is_empty() {
        println!("cargo:rustc-link-lib=dylib=c++"); // link to the C++ standard library
        println!("cargo:rustc-link-search=native={}/build", dst.display());
        println!("cargo:rustc-link-lib=static=hasherkawpow");
    }
}
