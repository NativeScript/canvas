package com.github.triniwiz.canvas;

import java.io.File;

/**
 * Created by triniwiz on 5/17/20
 */
public class FileReader {
    private static native byte[] nativeRead(String file);

    public static byte[] read(File file) {
        return read(file.getAbsolutePath());
    }

    public static byte[] read(String file) {
        return nativeRead(file);
    }
}
