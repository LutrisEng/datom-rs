// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

package engineering.lutris.datom;

public class Connection implements AutoCloseable {
    private static class JNI {
        static {
            DatomJNI.ensureLoaded();
        }

        private static native long create();

        private static native void destroy(long connection);

        private static native long latestT(long connection);
    }

    private long impl = 0;

    public Connection() {
        impl = JNI.create();
    }

    public long latestT() {
        return JNI.latestT(impl);
    }

    @Override
    public void close() {
        JNI.destroy(impl);
    }
}
