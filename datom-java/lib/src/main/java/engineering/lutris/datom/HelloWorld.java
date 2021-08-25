// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>
package engineering.lutris.datom;

public class HelloWorld {
    public static native String hello(String input);

    static {
        System.loadLibrary("datom_java");
    }
}
