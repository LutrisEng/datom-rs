// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

package engineering.lutris.datom;

public class Fact implements AutoCloseable {
    private static class JNI {
        static {
            DatomJNI.ensureLoaded();
        }

        private static native long fromEdn(String edn);

        private static native void destroy(long fact);

        private static native String toEdn(long fact);
    }

    private long impl = 0;

    private Fact(long impl) {
        this.impl = impl;
    }

    public static Fact fromEdn(String edn) {
        return new Fact(JNI.fromEdn(edn));
    }

    public String toEdn() {
        return JNI.toEdn(this.impl);
    }

    @Override
    public void close() {
        JNI.destroy(impl);
    }
}
