fn main() {
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        println!("cargo:rustc-link-lib=framework=Accelerate")
    }
}
