package com.github.triniwiz.canvas;

/**
 * Created by triniwiz on 5/15/20
 */
public class TextEncoder {
    private long nativeEncoder = 0;

    private static native long nativeInit(String encoding);

    private static native String nativeGetEncoding(long encoder);

    private static native byte[] nativeEncode(long encoder, String text);

    public TextEncoder() {
        init("utf-8");
    }

    public TextEncoder(String encoding) {
        init(encoding);
    }

    private void init(String encoding) {
        nativeEncoder = nativeInit(encoding);
    }

    public String getEncoding() {
        return nativeGetEncoding(nativeEncoder);
    }

    public byte[] encode(String text) {
        return nativeEncode(nativeEncoder, text);
    }
}
