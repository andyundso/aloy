// Stolen fro https://github.com/mockiato/mockiato/blob/master/build.rs

use rustc_version::{version_meta, Channel};

fn main() {
    if let Channel::Nightly = version_meta().unwrap().channel {
        println!("cargo:rustc-cfg=rustc_is_nightly");
    }
}
