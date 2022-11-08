// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

package engineering.lutris.datom;

import java.io.File;
import java.io.FileOutputStream;
import java.io.FileWriter;
import java.io.IOException;
import java.io.InputStream;

import org.apache.commons.lang3.SystemUtils;

class DatomJNI {
    private static class MissingNativesException extends Exception {
        MissingNativesException(String libraryName) {
            super(String.format("Missing native library named %s in classpath", libraryName));
        }
    }

    private static boolean loaded = false;

    private static String getLibraryExtension() throws InvalidPlatformException {
        if (SystemUtils.IS_OS_WINDOWS) {
            return "dll";
        } else if (SystemUtils.IS_OS_MAC) {
            return "dylib";
        } else if (SystemUtils.IS_OS_LINUX) {
            return "so";
        } else {
            throw new InvalidPlatformException(System.getProperty("os.name"));
        }
    }

    private static String getLibraryBasename() throws InvalidPlatformException {
        if (SystemUtils.IS_OS_WINDOWS) {
            return "datom_java";
        } else if (SystemUtils.IS_OS_MAC || SystemUtils.IS_OS_LINUX) {
            return "libdatom_java";
        } else {
            throw new InvalidPlatformException(System.getProperty("os.name"));
        }
    }

    private static String getLibraryName() throws InvalidPlatformException {
        return String.format("%s.%s", getLibraryBasename(), getLibraryExtension());
    }

    private static String getPlatformName() throws InvalidPlatformException {
        if (SystemUtils.IS_OS_WINDOWS) {
            return "windows";
        } else if (SystemUtils.IS_OS_MAC) {
            return "macos";
        } else if (SystemUtils.IS_OS_LINUX) {
            return "linux";
        } else {
            throw new InvalidPlatformException(System.getProperty("os.name"));
        }
    }

    private static String getLibraryPath() throws InvalidPlatformException {
        String fromProperty = System.getProperty("engineering.lutris.datom.nativeResourcePath");
        if (fromProperty != null) {
            return String.format("%s/%s", fromProperty, getLibraryName());
        } else {
            return String.format("natives/%s/%s/%s", getPlatformName(), System.getProperty("os.arch"),
                    getLibraryName());
        }
    }

    private static void load() throws IOException, InvalidPlatformException, MissingNativesException {
        String libraryPath = getLibraryPath();
        String libraryBasename = getLibraryBasename();
        String libraryExtension = getLibraryExtension();
        File nativeTempFile = File.createTempFile(libraryBasename, libraryExtension);
        try (InputStream nativeInput = DatomJNI.class.getResourceAsStream(libraryPath)) {
            if (nativeInput == null) {
                throw new MissingNativesException(libraryPath);
            }
            try (FileOutputStream nativeOutput = new FileOutputStream(nativeTempFile)) {
                nativeInput.transferTo(nativeOutput);
            }
        }
        System.load(nativeTempFile.getAbsolutePath());
    }

    static synchronized void ensureLoaded() {
        if (!loaded) {
            try {
                load();
            } catch (Exception e) {
                System.err.println("Failed to load engineering.lutris.datom native library");
                e.printStackTrace();
                System.exit(1);
            }
        }
    }
}
