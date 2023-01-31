extern crate bindgen;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    

    println!("cargo:rustc-link-search=native=C:/OpenIsobus/isobus-plus-plus-sys/ISO11783-CAN-Stack/build/isobus");
    // println!("cargo:rustc-link-search=native=C:/OpenIsobus/isobus-plus-plus-sys/ISO11783-CAN-Stack/build/hardware_integration/");
    // println!("cargo:rustc-link-search=native=C:/OpenIsobus/isobus-plus-plus-sys/ISO11783-CAN-Stack/build/utility/");
    println!("cargo:rustc-link-lib=Isobus");
    // println!("cargo:rustc-link-lib=HardwareIntegration");
    // println!("cargo:rustc-link-lib=SystemTiming");
    println!("cargo:rerun-if-changed=wrapper.hpp");

    // // Run `cmake` to compile.
    // if !std::process::Command::new("cmake")
    //     .arg("-SISO11783-CAN-Stack/.")
    //     .arg("-BISO11783-CAN-Stack/build")
    //     .output()
    //     .expect("could not spawn `cmake`")
    //     .status
    //     .success()
    // {
    //     // Panic if the command was not successful.
    //     panic!("could not compile object file");
    // }
    // if !std::process::Command::new("cmake")
    //     // .arg("-T ClangCL")
    //     .arg("--build ISO11783-CAN-Stack/build")
    //     .output()
    //     .expect("could not spawn `cmake`")
    //     .status
    //     .success()
    // {
    //     // Panic if the command was not successful.
    //     // panic!("could not compile object file");
    // }

    // let bindings = bindgen::Builder::default()
    //     .clang_arg("-IISO11783-CAN-Stack/isobus/include")
    //     // .clang_arg("-IISO11783-CAN-Stack/utility/include")
    //     // .clang_arg("-IISO11783-CAN-Stack/hardware_integration/include")
    //     // .clang_arg("-std=c++14")
    //     .opaque_type("std::.*")
    //     .header("wrapper.hpp")
    //     .generate()
    //     .expect("Unable to generate bindings");


    // bindings
    //     .write_to_file("src/bindings.rs")
    //     .expect("Couldn't write bindings!");
}