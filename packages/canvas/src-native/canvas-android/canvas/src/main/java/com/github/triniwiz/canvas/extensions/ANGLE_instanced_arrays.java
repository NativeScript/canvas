package com.github.triniwiz.canvas.extensions;

import android.opengl.GLES11Ext;
import android.opengl.GLES20;
import android.opengl.GLES30;
import android.os.Build;

import androidx.annotation.RequiresApi;

import com.github.triniwiz.canvas.WebGLRenderingContext;

/**
 * Created by triniwiz on 5/8/20
 */
@RequiresApi(api = Build.VERSION_CODES.JELLY_BEAN_MR2)
public class ANGLE_instanced_arrays {

    public final int VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE = GLES30.GL_VERTEX_ATTRIB_ARRAY_DIVISOR;

    public void drawArraysInstancedANGLE(int mode, int first, int count, int primcount) {
        GLES30.glDrawArraysInstanced(mode, first, count, primcount);
    }

    public void drawElementsInstancedANGLE(int mode, int count, int type, int offset, int primcount) {
        GLES30.glDrawElementsInstanced(mode, count, type, offset, primcount);
    }

    public void vertexAttribDivisorANGLE(int index, int divisor) {
        GLES30.glVertexAttribDivisor(index, divisor);
    }
}
