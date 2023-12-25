use cmake::Config;

fn main() {
    std::process::Command::new("make")
        .args(["-Clibrime", "deps"])
        .output()
        .expect("librime make deps failed.");

    let dst = Config::new("librime")
        .define("BUILD_TEST", "0")
        .define("BUILD_SHARED_LIBS", "1")
        .define("BUILD_STATIC", "0")
        .define("CMAKE_BUILD_TYPE", "Release")
        .build();

    let bindings = bindgen::Builder::default()
        .clang_args(&["-x", "c"])
        .clang_args(&[format!("-I{}/include", dst.display())])
        .header("wrapper.h")
        .generate()
        .expect("unable to generate rime_api.h bindings!");

    let out = std::path::PathBuf::from("./src");
    bindings
        .write_to_file(out.join("api.rs"))
        .expect("could not write bindings!");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=dylib=rime");
}
