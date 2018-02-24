extern crate cc;

fn main() {
    cc::Build::new()
        .file("sys/aesb.c")
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
        .file("sys/random.c")
        .file("sys/skein.c")
        .file("sys/slow-hash.c")
        .file("sys/tree-hash.c")
        .include("sys")
        .compile("cncrypto");
}
