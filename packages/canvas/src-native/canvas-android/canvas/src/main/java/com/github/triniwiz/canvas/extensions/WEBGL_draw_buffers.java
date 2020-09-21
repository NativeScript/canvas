package com.github.triniwiz.canvas.extensions;


import android.opengl.GLES30;
import android.os.Build;

import androidx.annotation.RequiresApi;

import com.github.triniwiz.canvas.Constants;

/**
 * Created by triniwiz on 5/1/20
 */

@RequiresApi(api = Build.VERSION_CODES.JELLY_BEAN_MR2)
public class WEBGL_draw_buffers {
    public int COLOR_ATTACHMENT0_WEBGL = Constants.GL_COLOR_ATTACHMENT0_EXT;
    public int COLOR_ATTACHMENT1_WEBGL = Constants.GL_COLOR_ATTACHMENT1_EXT;
    public int COLOR_ATTACHMENT2_WEBGL = Constants.GL_COLOR_ATTACHMENT2_EXT;
    public int COLOR_ATTACHMENT3_WEBGL = Constants.GL_COLOR_ATTACHMENT3_EXT;
    public int COLOR_ATTACHMENT4_WEBGL = Constants.GL_COLOR_ATTACHMENT4_EXT;
    public int COLOR_ATTACHMENT5_WEBGL = Constants.GL_COLOR_ATTACHMENT5_EXT;
    public int COLOR_ATTACHMENT6_WEBGL = Constants.GL_COLOR_ATTACHMENT6_EXT;
    public int COLOR_ATTACHMENT7_WEBGL = Constants.GL_COLOR_ATTACHMENT7_EXT;
    public int COLOR_ATTACHMENT8_WEBGL = Constants.GL_COLOR_ATTACHMENT8_EXT;
    public int COLOR_ATTACHMENT9_WEBGL = Constants.GL_COLOR_ATTACHMENT9_EXT;
    public int COLOR_ATTACHMENT10_WEBGL = Constants.GL_COLOR_ATTACHMENT10_EXT;
    public int COLOR_ATTACHMENT11_WEBGL = Constants.GL_COLOR_ATTACHMENT11_EXT;
    public int COLOR_ATTACHMENT12_WEBGL = Constants.GL_COLOR_ATTACHMENT12_EXT;
    public int COLOR_ATTACHMENT13_WEBGL = Constants.GL_COLOR_ATTACHMENT13_EXT;
    public int COLOR_ATTACHMENT14_WEBGL = Constants.GL_COLOR_ATTACHMENT14_EXT;
    public int COLOR_ATTACHMENT15_WEBGL = Constants.GL_COLOR_ATTACHMENT15_EXT;
    public int DRAW_BUFFER0_WEBGL = Constants.GL_DRAW_BUFFER0_EXT;
    public int DRAW_BUFFER1_WEBGL = Constants.GL_DRAW_BUFFER1_EXT;
    public int DRAW_BUFFER2_WEBGL = Constants.GL_DRAW_BUFFER2_EXT;
    public int DRAW_BUFFER3_WEBGL = Constants.GL_DRAW_BUFFER3_EXT;
    public int DRAW_BUFFER4_WEBGL = Constants.GL_DRAW_BUFFER4_EXT;
    public int DRAW_BUFFER5_WEBGL = Constants.GL_DRAW_BUFFER5_EXT;
    public int DRAW_BUFFER6_WEBGL = Constants.GL_DRAW_BUFFER6_EXT;
    public int DRAW_BUFFER7_WEBGL = Constants.GL_DRAW_BUFFER7_EXT;
    public int DRAW_BUFFER8_WEBGL = Constants.GL_DRAW_BUFFER8_EXT;
    public int DRAW_BUFFER9_WEBGL = Constants.GL_DRAW_BUFFER9_EXT;
    public int DRAW_BUFFER10_WEBGL = Constants.GL_DRAW_BUFFER10_EXT;
    public int DRAW_BUFFER11_WEBGL = Constants.GL_DRAW_BUFFER11_EXT;
    public int DRAW_BUFFER12_WEBGL = Constants.GL_DRAW_BUFFER12_EXT;
    public int DRAW_BUFFER13_WEBGL = Constants.GL_DRAW_BUFFER13_EXT;
    public int DRAW_BUFFER14_WEBGL = Constants.GL_DRAW_BUFFER14_EXT;
    public int DRAW_BUFFER15_WEBGL = Constants.GL_DRAW_BUFFER15_EXT;
    public int MAX_COLOR_ATTACHMENTS_WEBGL = Constants.GL_MAX_COLOR_ATTACHMENTS_EXT;
    public int MAX_DRAW_BUFFERS_WEBGL = Constants.GL_MAX_DRAW_BUFFERS_EXT;

    public void drawBuffersWEBGL(int[] buffers) {
        GLES30.glDrawBuffers(buffers.length, buffers, 0);
    }
}
