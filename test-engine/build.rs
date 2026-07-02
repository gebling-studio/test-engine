fn main() {
    plat::platforms();

    // The inspect module compiles in only under debug_assertions. A release
    // profile with debug-assertions enabled would ship it, so refuse to build.
    let release = std::env::var("PROFILE").as_deref() == Ok("release");
    let debug_assertions = std::env::var_os("CARGO_CFG_DEBUG_ASSERTIONS").is_some();

    assert!(
        !(release && debug_assertions),
        "inspect is debug-only. Do not enable debug-assertions in a release profile."
    );
}
