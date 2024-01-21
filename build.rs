use cmake::Config;

fn main() {
    std::process::Command::new("make")
        .args(["-Clibrime", "deps"])
        .output()
        .expect("librime make deps failed.");

    #[cfg(feature = "logging")]
    let rime_logging_switch = "on";
    #[cfg(not(feature = "logging"))]
    let rime_logging_switch = "off";

    let dst = Config::new("librime")
        .define("BUILD_TEST", "off")
        .define("ENABLE_LOGGING", rime_logging_switch)
        .define("BUILD_SHARED_LIBS", "off")
        .define("BUILD_STATIC", "on")
        .build();

    let bindings = bindgen::Builder::default()
        .clang_args(&["-x", "c++"])
        .clang_args(&[format!("-I{}/include", dst.display())])
        .header("cbits/bindings.h")
        .generate()
        .expect("unable to generate rime_api.h bindings!");

    let out = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::PathBuf::from(&out);
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("could not write bindings!");

    cc::Build::new()
        .cpp(true)
        .flag("-Wno-missing-field-initializers")
        .flag("-Icbits")
        .flag(&format!("-I{}/include", dst.display()))
        .file("cbits/bindings.cpp")
        .compile("bindings");

    #[allow(unused_mut, unused_assignments)]
    let mut selected = false;

    #[cfg(all(feature = "link-cxx", not(feature = "link-stdcxx")))]
    {
        println!("cargo:rustc-flags=-l dylib=c++");
        selected = true;
    }
    #[cfg(all(feature = "link-stdcxx", not(feature = "link-cxx")))]
    {
        println!("cargo:rustc-flags=-l dylib=stdc++");
        selected = true;
    }

    if !selected {
        #[cfg(target_os = "macos")]
        println!("cargo:rustc-flags=-l dylib=c++");
        #[cfg(target_os = "linux")]
        println!("cargo:rustc-flags=-l dylib=stdc++");
    }

    let cur = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/librime/lib", cur);
    println!("cargo:rustc-link-lib=static=rime");
    println!("cargo:rustc-link-lib=static=glog");
    println!("cargo:rustc-link-lib=static=yaml-cpp");
    println!("cargo:rustc-link-lib=static=marisa");
    println!("cargo:rustc-link-lib=static=opencc");
    println!("cargo:rustc-link-lib=static=leveldb");
    println!("cargo:rustc-link-lib=static=bindings");

    println!("cargo:rerun-if-changed=cbits/bindings.h");
    println!("cargo:rerun-if-changed=cbits/bindings.cpp");
}
