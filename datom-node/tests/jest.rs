// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::process::Command;

#[test]
fn jest() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(windows) {
        let status = Command::new("powershell")
            .arg("-Command")
            .arg("npm ci")
            .status()?;
        assert!(status.success());
        let status = Command::new("powershell")
            .arg("-Command")
            .arg("npm run jest")
            .status()?;
        assert!(status.success());
    } else {
        let status = Command::new("npm").arg("ci").status()?;
        assert!(status.success());
        let status = Command::new("npm").arg("run").arg("jest").status()?;
        assert!(status.success());
    }
    Ok(())
}
