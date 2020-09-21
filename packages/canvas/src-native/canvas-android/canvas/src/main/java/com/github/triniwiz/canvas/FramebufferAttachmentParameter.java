package com.github.triniwiz.canvas;

/**
 * Created by triniwiz on 4/21/20
 */
public class FramebufferAttachmentParameter {
    boolean isTexture = false;
    boolean isRenderbuffer = false;
    int value = 0;

    public FramebufferAttachmentParameter() {}
    public FramebufferAttachmentParameter(boolean isTexture, boolean isRenderbuffer, int value) {
        this.isTexture = isTexture;
        this.isRenderbuffer = isRenderbuffer;
        this.value = value;
    }

    public int getValue() {
        return value;
    }

    public boolean isRenderbuffer() {
        return isRenderbuffer;
    }

    public boolean isTexture() {
        return isTexture;
    }
}
