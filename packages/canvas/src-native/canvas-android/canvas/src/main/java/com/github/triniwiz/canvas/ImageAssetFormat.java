package com.github.triniwiz.canvas;

/**
 * Created by triniwiz on 5/4/20
 */
public enum ImageAssetFormat {
    JPG(0),
    PNG(1),
    ICO(2),
    BMP(3),
    TIFF(4);
    int format;
    ImageAssetFormat(int format){
        this.format = format;
    }

    public int getFormat() {
        return format;
    }
}
