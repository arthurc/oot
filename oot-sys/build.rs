use cc;
use glob;

fn main() {
    let mut build = cc::Build::new();

    build
        .include("oot/src")
        .include("oot/include")
        .flag("-m32")
        .flag_if_supported("-Wno-unused-variable")
        .flag_if_supported("-Wno-unknown-pragmas")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-variable")
        .flag_if_supported("-Wno-missing-braces")
        .define("_LANGUAGE_C", None);

    for file in glob::glob("oot/src/**/*.c").unwrap() {
        build.file(file.unwrap());
    }
    
    build.compile("oot");
}
