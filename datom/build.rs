// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

#[cfg(feature = "c")]
extern crate cbindgen;

fn main() {
    #[cfg(feature = "c")]
    {
        use std::env;

        let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let profile = env::var("PROFILE").unwrap();

        cbindgen::Builder::new()
            .with_crate(crate_dir)
            .with_language(cbindgen::Language::C)
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(format!("../target/{}/datom.h", profile));
    }
}
