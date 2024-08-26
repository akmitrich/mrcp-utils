// Copyright 2024 ООО Оптимумсити

//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at

//        http://www.apache.org/licenses/LICENSE-2.0

//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

fn main() {
    println!("cargo:rerun-if-changed=include/mrcp.h");
    let unimrcp_path =
        std::env::var("UNIMRCP_PATH").unwrap_or_else(|_| "/usr/local/unimrcp".into());
    let apr_lib_path = std::env::var("APR_LIB_PATH").unwrap_or_else(|_| "/usr/local/apr".into());
    let apr_include_path =
        std::env::var("APR_INCLUDE_PATH").unwrap_or_else(|_| "/usr/local/apr".into());

    println!("cargo:rustc-link-lib=apr-1");
    println!("cargo:rustc-link-lib=unimrcpserver");
    println!("cargo:rustc-link-search={}/lib", unimrcp_path);
    println!("cargo:rustc-link-search={}/lib", apr_lib_path);

    let mut builder = bindgen::Builder::default();
    builder = builder
        .header("include/mrcp.h")
        .clang_arg(format!("-I{}/include", unimrcp_path))
        .clang_arg(format!("-I{}/include/apr-1", apr_include_path));
    let bindings = builder
        .constified_enum_module("*")
        .prepend_enum_name(false)
        // The problem with generating `FALSE`
        // it is generated not as `apt_bool_t` but as `u32`
        // so it had to be defined explicitly.
        .blocklist_item("FALSE")
        // .blacklist_item("FALSE")
        .derive_eq(true)
        .generate()
        .expect("Unable to generate bindings.");
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings.");
}
