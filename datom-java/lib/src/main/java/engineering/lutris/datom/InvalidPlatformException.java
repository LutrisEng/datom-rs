// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

package engineering.lutris.datom;

public class InvalidPlatformException extends Exception {
    InvalidPlatformException(String platform) {
        super(String.format("The current platform (%s) isn't supported by engineering.lutris.datom.", platform));
    }
}
