<!-- SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc -->
<!-- SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent -->
<!-- SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering> -->

# datom-rs

<a href="https://github.com/LutrisEng/datom-rs/actions">
    <img src="https://img.shields.io/github/workflow/status/LutrisEng/datom-rs/CI"
        alt="CI status" />
</a>
<a href="https://crates.io/crates/datom">
    <img src="https://img.shields.io/crates/v/datom" alt="Version" />
</a>
<a href="https://libraries.io/cargo/datom">
    <img src="https://img.shields.io/librariesio/release/cargo/datom"
        alt="Libraries.io dependency status for latest release" />
</a>
<a href="https://app.fossa.com/projects/git%2Bgithub.com%2FLutrisEng%2Fdatom-rs?ref=badge_shield">
    <img src="https://app.fossa.com/api/projects/git%2Bgithub.com%2FLutrisEng%2Fdatom-rs.svg?type=shield"
        alt="FOSSA Status">
</a>
<a href="https://blueoakcouncil.org/license/1.0.0">
    <img src="https://img.shields.io/badge/license-BlueOak%2FBSD-blue.svg" alt="License" />
</a>
<a href="https://matrix.to/#/#datom-rs:lutris.engineering">
    <img src="https://img.shields.io/badge/chat-%23datom--rs%3Alutris.engineering-informational"
        alt="Matrix: #datom-rs:lutris.engineering" />
</a>
<a href="https://web.libera.chat/?channel=##datom-rs">
    <img src="https://img.shields.io/badge/libera.chat-%23%23datom--rs-informational"
        alt="IRC: ##datom.rs on libera.chat" />
</a>
<a href="https://github.com/sponsors/LutrisEng">
    <img src="https://img.shields.io/github/sponsors/LutrisEng" alt="GitHub Sponsors" />
</a>
<a href="https://github.com/LutrisEng/datom-rs/blob/main/CODE_OF_CONDUCT.md">
    <img src="https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg"
        alt="Contributor Covenant" />
</a>

## An open-source database inspired by Datomic

[Documentation](https://os.lutris.engineering/datom-rs/datom)

datom-rs is currently under pre-release development.

## Installation

Add the following to the `[dependencies]` section of your `Cargo.toml`:

```toml
datom = "0.1"
```

## MSRV

Currently, datom-rs requires Rust nightly. We use the following unstable features, though we're open to refactoring them out.

```rust
#![feature(generic_associated_types, map_first_last)]
```

## Sponsors

<table class="pure-table pure-table-horizontal">
    <thead>
        <tr>
            <th>Sponsor</th>
            <th>Contribution</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>
                <a href="https://lutris.engineering/?utm_source=lutrisengineering&utm_medium=github&utm_campaign=datom-rs"
                    title="Lutris Engineering, Inc">
                    <img alt="Lutris Engineering"
                        src="https://user-images.githubusercontent.com/1830959/129986000-d00e9309-a657-40a0-8cf4-518a5cd7dfae.png"
                        width="400" />
                </a>
            </td>
            <td>Lutris Engineering runs the datom-rs project.</td>
        </tr>
    </tbody>
</table>

