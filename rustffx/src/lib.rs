#![allow(dead_code)]

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

// para manter o nome no crates.io
