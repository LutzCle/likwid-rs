fn main() {
    #[cfg(feature = "likwid_perfmon")]
    {
        println!("cargo:rustc-link-search=native=/usr/local");
        println!("cargo:rustc-link-lib=likwid");
    }
}
