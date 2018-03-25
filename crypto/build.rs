extern crate cc;

fn main() {
    let mut build = cc::Build::new();

    let tool = build.get_compiler();

    if tool.is_like_gnu() || tool.is_like_clang() {
        build.flag_if_supported("-std=c99");
        build.flag_if_supported("-msse4.1")
            .flag_if_supported("-maes");
    }

    if tool.is_like_msvc() {   
        build.include("sys/mvsc");
    }

    build.warnings(false);

    build.file("sys/aesb.c")
        .file("sys/blake256.c")
        .file("sys/crypto-ops-data.c")
        .file("sys/crypto-ops.c")
        .file("sys/groestl.c")
        .file("sys/hash-extra-blake.c")
        .file("sys/hash-extra-groestl.c")
        .file("sys/hash-extra-jh.c")
        .file("sys/hash-extra-skein.c")
        .file("sys/hash.c")
        .file("sys/jh.c")
        .file("sys/keccak.c")
        .file("sys/oaes_lib.c")
        .file("sys/skein.c")
        .file("sys/slow-hash.c")
        .include("sys");

    build.compile("cncrypto");
}
