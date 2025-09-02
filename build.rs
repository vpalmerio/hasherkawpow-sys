use cmake::{self, Config};

fn main() {

    let dst = 
        Config::new("src/src")
            .build_target("libhasherkawpow")
            .very_verbose(true)
            .always_configure(false)
            .build();
    
    let toolchain_environment = std::env::var("CARGO_CFG_TARGET_ENV").unwrap();

    if toolchain_environment == "gnu" {
        println!("cargo:rustc-link-search=all={}/build", dst.display());
        println!("cargo:rustc-link-lib=static=hasherkawpow");
    } else if toolchain_environment == "msvc" {
        println!("cargo:rustc-link-search=all={}/build/Debug", dst.display());
        println!("cargo:rustc-link-lib=static=libhasherkawpow");
    } else if toolchain_environment == "" {
        println!("cargo:rustc-link-search=all={}/build/CMakeFiles/libhasherkawpow.dir/", dst.display());
        println!("cargo:rustc-link-lib=static=libhasherkawpow.cc.o");
        println!("cargo:rustc-link-lib=static=uint256.cpp.o");
        println!("cargo:rustc-link-search=all={}/build/CMakeFiles/libhasherkawpow.dir/ethash", dst.display());
        println!("cargo:rustc-link-lib=static=ethash.cpp.o");
        println!("cargo:rustc-link-lib=static=managed.cpp.o");
        println!("cargo:rustc-link-lib=static=primes.c.o");
        println!("cargo:rustc-link-lib=static=progpow.cpp.o");
        println!("cargo:rustc-link-search=all={}/build/CMakeFiles/libhasherkawpow.dir/keccak", dst.display());
        println!("cargo:rustc-link-lib=static=keccak.c.o");
        println!("cargo:rustc-link-lib=static=keccakf800.c.o");
        println!("cargo:rustc-link-lib=static=keccakf1600.c.o");
    }
}