package com.github.triniwiz.canvas;

/**
 * Created by triniwiz on 2019-07-13
 */
public enum CanvasTextBaseline {
    Top("top"),
    Hanging("hanging"),
    Middle("middle"),
    Alphabetic("alphabetic"),
    Ideographic("ideographic"),
    Bottom("bottom");
    String baseLine;

    CanvasTextBaseline(String baseLine) {
        this.baseLine = baseLine;
    }

    @Override
    public String toString() {
        return baseLine;
    }}
