package com.github.triniwiz.canvas;

/**
 * Created by triniwiz on 5/30/20
 */
public class Color implements ICanvasColorStyle {
    int color;

    public Color(int color) {
        this.color = color;
    }

    public Color(String color) {
        this.color = Colors.getColor(color);
    }

    @Override
    public CanvasColorStyleType getStyleType() {
        return CanvasColorStyleType.Color;
    }
}
