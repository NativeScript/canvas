package com.github.triniwiz.canvas.extensions;

import android.opengl.GLES20;

import com.github.triniwiz.canvas.Constants;

import static android.opengl.GLES30.GL_R11F_G11F_B10F;

/**
 * Created by triniwiz on 5/1/20
 */
public class EXT_color_buffer_float {
    public int R16F = Constants.GL_R16F_EXT;

    public int RG16F = Constants.GL_RG16F_EXT;

    public int RGB16F = Constants.GL_RGB16F_EXT;

    public int R32F  = Constants.GL_R32F_EXT;

    public int RG32F = Constants.GL_RG32F_EXT;

    public int RGBA32F = Constants.GL_RGBA32F_EXT;

    public int R11F_G11F_B10F = GL_R11F_G11F_B10F;

}
