package com.github.triniwiz.canvas;

/**
 * Created by triniwiz on 5/1/20
 */
public class IndexedParameter {
    boolean isBuffer = false;
    int bufferValue = 0;
    long value = -1;

    public boolean getIsBuffer() {
        return isBuffer;
    }

    public int getBufferValue() {
        return bufferValue;
    }

    public long getValue() {
        return value;
    }
}
