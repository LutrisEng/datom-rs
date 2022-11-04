// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

package engineering.lutris.datom;

class DatomJNI {
    private static boolean loaded = false;

    static synchronized void ensureLoaded() {
        if (!loaded) {
            System.loadLibrary("datom_java");
        }
    }
}
