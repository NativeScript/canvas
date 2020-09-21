package com.github.triniwiz.canvas;

/**
 * Created by triniwiz on 2019-07-13
 */
public enum CanvasTextAlignment {
    Left("left"),
    Right("right"),
    Center("center"),
    Start("start"),
    End("end");
    String textAlign;

    CanvasTextAlignment(String textAlign) {
        this.textAlign = textAlign;
    }

    @Override
    public String toString() {
        return textAlign;
    }}
