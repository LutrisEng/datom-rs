// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::process::Command;

fn command_wrapper() -> Command {
    if cfg!(windows) {
        let mut command = Command::new("powershell");
        command.arg("-Command");
        command
    } else {
        let mut command = Command::new("sh");
        command.arg("-c");
        command
    }
}

fn run_command(command: &str) {
    assert!(
        command_wrapper().arg(command).status().unwrap().success(),
        "Command {:#?} failed",
        command
    );
}

#[test]
fn jest() {
    run_command("npm ci");
    run_command("npm run jest");
}
