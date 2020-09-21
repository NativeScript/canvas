package com.github.triniwiz.canvas;

/**
 * Created by triniwiz on 2019-07-13
 */
public enum CanvasColorStyleType {
    Color("color"),
    Gradient("gradient"),
    Pattern("pattern");
    private String type;

    CanvasColorStyleType(String type) {
        this.type = type;
    }

    @Override
    public String toString() {
        return type;
    }}
