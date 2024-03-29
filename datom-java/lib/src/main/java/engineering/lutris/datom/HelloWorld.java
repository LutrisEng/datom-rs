// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

package engineering.lutris.datom;

public class HelloWorld {
    private static class JNI {
        static {
            DatomJNI.ensureLoaded();
        }

        private static native String hello(String world);
    }

    public static String hello(String world) {
        return JNI.hello(world);
    }
}
