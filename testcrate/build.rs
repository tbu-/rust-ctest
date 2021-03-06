extern crate ctest;
extern crate cc;

fn main() {
    cc::Build::new()
        .include("src")
        .warnings(false)
        .file("src/t1.c")
        .compile("libt1.a");
    cc::Build::new()
        .warnings(false)
        .file("src/t2.c")
        .compile("libt2.a");
    ctest::TestGenerator::new()
                         .header("t1.h")
                         .include("src")
                         .fn_cname(|a, b| b.unwrap_or(a).to_string())
                         .type_name(move |ty, is_struct, is_union|
                             match ty {
                                 "T1Union" => ty.to_string(),
                                 t if is_struct => format!("struct {}", t),
                                 t if is_union => format!("union {}", t),
                                 t => t.to_string(),
                             }
                         )
                         .generate("src/t1.rs", "t1gen.rs");
    ctest::TestGenerator::new()
                         .header("t2.h")
                         .include("src")
                         .type_name(move |ty, is_struct, is_union|
                             match ty {
                                 "T2Union" => ty.to_string(),
                                 t if is_struct => format!("struct {}", t),
                                 t if is_union => format!("union {}", t),
                                 t => t.to_string(),
                             }
                         )
                         .generate("src/t2.rs", "t2gen.rs");
}
