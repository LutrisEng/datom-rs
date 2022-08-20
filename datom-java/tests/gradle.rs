// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::process::Command;

#[test]
fn gradle() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(windows) {
        let status = Command::new(".\\gradlew.bat").arg("test").status()?;
        assert!(status.success());
    } else {
        let status = Command::new("./gradlew").arg("test").status()?;
        assert!(status.success());
    }
    Ok(())
}
