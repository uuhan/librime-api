use cmake::Config;
use std::path::PathBuf;

fn main() {
    let dst = select_librime();

    let bindings = bindgen::Builder::default()
        .clang_args(&["-x", "c"])
        .clang_args(&[format!("-I{}/include", dst.display())])
        .header("wrapper.h")
        .generate()
        .expect("unable to generate rime_api.h bindings!");

    let out = std::env::var("OUT_DIR").unwrap();
    let out_dir = PathBuf::from(&out);
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("could not write bindings!");

    #[allow(unused_mut, unused_assignments)]
    let mut selected = false;

    #[cfg(all(feature = "link-cxx", not(feature = "link-stdcxx")))]
    {
        println!("cargo::rustc-flags=-l dylib=c++");
        selected = true;
    }
    #[cfg(all(feature = "link-stdcxx", not(feature = "link-cxx")))]
    {
        println!("cargo::rustc-flags=-l dylib=stdc++");
        selected = true;
    }

    if !selected {
        #[cfg(target_os = "macos")]
        println!("cargo::rustc-flags=-l dylib=c++");
        #[cfg(target_os = "linux")]
        println!("cargo::rustc-flags=-l dylib=stdc++");
    }
}

fn select_librime() -> PathBuf {
    let mut dst = PathBuf::new();

    if cfg!(not(feature = "prebuilt")) {
        return compile_librime();
    }

    if cfg!(target_os = "windows") {
        dst.push(format!(
            "{}/msvc/x64/1.11.2/dist",
            std::env::current_dir().unwrap().display()
        ));
        println!("cargo::rustc-link-search=native={}/lib", dst.display());
        println!("cargo::rustc-link-lib=static=rime");
    } else if cfg!(target_os = "macos") {
        dst.push(format!(
            "{}/macos/universal/1.11.2/dist",
            std::env::current_dir().unwrap().display()
        ));
        println!("cargo::rustc-link-search=native={}/lib", dst.display());
        println!("cargo::rustc-link-lib=rime");
    } else {
        println!("cargo::warning=Linux Prebuilt **NOT** Supplied");
        return compile_librime();
    }

    dst
}

fn compile_librime() -> PathBuf {
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

    println!("cargo::rustc-link-search=native={}/lib", dst.display());
    println!("cargo::rustc-link-lib=static=rime");

    println!("cargo::rustc-link-search=native=./librime/lib");
    println!("cargo::rustc-link-lib=static=glog");
    println!("cargo::rustc-link-lib=static=yaml-cpp");
    println!("cargo::rustc-link-lib=static=marisa");
    println!("cargo::rustc-link-lib=static=opencc");
    println!("cargo::rustc-link-lib=static=leveldb");

    dst
}
