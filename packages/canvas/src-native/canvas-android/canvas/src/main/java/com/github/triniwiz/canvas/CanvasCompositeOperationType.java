package com.github.triniwiz.canvas;

/**
 * Created by triniwiz on 2019-07-13
 */
public enum CanvasCompositeOperationType {
    SourceOver("source-over"),
    SourceIn("source-in"),
    SourceoOut("source-out"),
    SourceAtop("source-atop"),
    DestinationOver("destination-over"),
    DestinationIn("destination-in"),
    DestinationOut("destination-out"),
    DestinationAtop("destination-atop"),
    Lighter("lighter"),
    Copy("copy"),
    Xor("xor"),
    Multiply("multiply"),
    Screen("screen"),
    Overlay("overlay"),
    Darken("darken"),
    Lighten("lighten"),
    ColorDodge("color-dodge"),
    ColorBurn("color-burn"),
    HardLight("hard-light"),
    SoftLight("soft-light"),
    Difference("difference"),
    Exclusion("exclusion"),
    Hue("hue"),
    Saturation("saturation"),
    Color("color"),
    Luminosity("luminosity");
    String type;

    CanvasCompositeOperationType(String type) {
        this.type = type;
    }

    @Override
    public String toString() {
        return type;
    }}
