fn main() {
    // println!("cargo home = {}",env!("CARGO_HOME"));
    // println!("cargo target dir = {}", env!("CARGO_TARGET_DIR")); // build.target-dir
    // println!("rustc = {}", env!("RUSTC"));
    // println!("rust doc = {}",env!("RUSTDOC"));
    // println!("rust doc flags = {}", env!("RUSTDOCFLAGS"));
    // println!("rust flags = {}", env!("RUSTFLAGS"));
    // println!("CARGO_INCREMENTAL = {}", env!("CARGO_INCREMENTAL"));
    // println!("CARGO_CACHE_RUSTC_INFO = {}", env!("CARGO_CACHE_RUSTC_INFO"));
    // println!("cargo name = {}", env!("CARGO_NAME"));
    // println!("CARGO_EMAIL = {}", env!("CARGO_EMAIL"));
    // println!("HTTPS_PROXY = {}", env!("HTTPS_PROXY"));
    // println!("HTTP_TIMEOUT  = {}", env!("HTTP_TIMEOUT"));
    // println!("TERM = {}",env!("TERM"));
    // println!("BROWSER = {}", env!("BROWSER"));

    // environment variable cargo sets for crates
    println!("CARGO = {}", env!("CARGO"));
    println!("CARGO_MANIFEST_DIR = {}", env!("CARGO_MANIFEST_DIR"));
    println!("CARGO_PKG_VERSION = {}", env!("CARGO_PKG_VERSION"));
    println!(
        "CARGO_PKG_VERSION_MAJOR = {}",
        env!("CARGO_PKG_VERSION_MAJOR")
    );
    println!(
        "CARGO_PKG_VERSION_MINOR = {}",
        env!("CARGO_PKG_VERSION_MINOR")
    );
    println!(
        "CARGO_PKG_VERSION_PATCH = {}",
        env!("CARGO_PKG_VERSION_PATCH")
    );
    println!("CARGO_PKG_VERSION_PRE = {}", env!("CARGO_PKG_VERSION_PRE"));
    println!("CARGO_PKG_AUTHORS = {}", env!("CARGO_PKG_AUTHORS"));
    println!("CARGO_PKG_NAME = {}", env!("CARGO_PKG_NAME"));
    println!("CARGO_PKG_DESCRIPTION = {}", env!("CARGO_PKG_DESCRIPTION"));
    println!("CARGO_PKG_HOMEPAGE = {}", env!("CARGO_PKG_HOMEPAGE"));
    println!("CARGO_PKG_REPOSITORY = {}", env!("CARGO_PKG_REPOSITORY"));
    println!("CARGO_PKG_LICENSE = {}", env!("CARGO_PKG_LICENSE"));
    println!(
        "CARGO_PKG_LICENSE_FILE = {}",
        env!("CARGO_PKG_LICENSE_FILE")
    );
    println!("CARGO_CRATE_NAME = {}", env!("CARGO_CRATE_NAME"));
    // println!("CARGO_BIN_NAME = {}", env!("CARGO_BIN_NAME"));
    // println!("OUT_DIR = {}", env!("OUT_DIR"));
    println!(
        "DYLD_FALLBACK_LIBRARY_PATH = {}",
        env!("DYLD_FALLBACK_LIBRARY_PATH")
    );
}
