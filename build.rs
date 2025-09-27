fn main() {
    // build libpsxav
    println!("cargo:rerun-if-changed=libpsxav/");
    cc::Build::new()
        .file("libpsxav/adpcm.c")
        .file("libpsxav/cdrom.c")
        .include("libpsxav")
        .compile("libpsxav");
}
