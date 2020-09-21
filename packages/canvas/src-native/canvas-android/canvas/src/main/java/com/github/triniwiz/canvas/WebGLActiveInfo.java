package com.github.triniwiz.canvas;

/**
 * Created by triniwiz on 4/21/20
 */
public class WebGLActiveInfo {
    String name = "";
    int size = 0;
    int type = 0;

    public WebGLActiveInfo() {
    }

    public WebGLActiveInfo(String name, int size, int type) {
        this.name = name;
        this.size = size;
        this.type = type;
    }

    public String getName() {
        return name;
    }

    public int getSize() {
        return size;
    }

    public int getType() {
        return type;
    }
}
