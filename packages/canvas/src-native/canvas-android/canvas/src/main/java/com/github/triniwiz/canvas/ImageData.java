package com.github.triniwiz.canvas;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;

/**
 * Created by triniwiz on 2019-08-04
 */
public class ImageData {
    private ByteBuffer data;
    private int width;
    private int height;

    ImageData(int width, int height, byte[] data) {
        this.width = width;
        this.height = height;
        this.data = ByteBuffer.allocate(data.length).order(ByteOrder.nativeOrder());
        this.data.put(data);
        this.data.rewind();
    }

    ImageData(int width, int height, ByteBuffer data) {
        this.width = width;
        this.height = height;
        this.data = data;
        this.data.rewind();
    }

    public int getHeight() {
        return height;
    }

    public int getWidth() {
        return width;
    }

    public ByteBuffer getData() {
        return data;
    }

}
