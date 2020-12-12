package com.github.triniwiz.canvas.extensions

import android.opengl.GLES30
import com.github.triniwiz.canvas.Constants

/**
 * Created by triniwiz on 5/1/20
 */
class EXT_color_buffer_float {
	var R16F = Constants.GL_R16F_EXT
	var RG16F = Constants.GL_RG16F_EXT
	var RGB16F = Constants.GL_RGB16F_EXT
	var R32F = Constants.GL_R32F_EXT
	var RG32F = Constants.GL_RG32F_EXT
	var RGBA32F = Constants.GL_RGBA32F_EXT
	var R11F_G11F_B10F = GLES30.GL_R11F_G11F_B10F
}
