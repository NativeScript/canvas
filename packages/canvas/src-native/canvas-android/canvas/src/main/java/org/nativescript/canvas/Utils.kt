package org.nativescript.canvas

import android.graphics.Bitmap
import android.graphics.SurfaceTexture
import android.opengl.GLES20
import android.os.Build
import android.os.Environment
import java.io.File
import java.nio.ByteBuffer


object Utils {
    private const val TAG = "Utils"

    @JvmStatic
    private external fun nativeGetBytesFromBitmap(bitmap: Bitmap?): ByteArray

    @JvmStatic
    private external fun nativeGetByteBufferFromBitmap(bitmap: Bitmap?): ByteBuffer

    @JvmStatic
    private external fun nativeMakeStateContextCurrent(state: Long): Boolean

    private var rating = -1
    val isEmulator: Boolean
        get() {
            var newRating = 0
            if (rating < 0) {
                if (Build.PRODUCT.contains("sdk") ||
                    Build.PRODUCT.contains("Andy") ||
                    Build.PRODUCT.contains("ttVM_Hdragon") ||
                    Build.PRODUCT.contains("google_sdk") ||
                    Build.PRODUCT.contains("Droid4X") ||
                    Build.PRODUCT.contains("nox") ||
                    Build.PRODUCT.contains("sdk_x86") ||
                    Build.PRODUCT.contains("sdk_gphone_x86_arm") ||
                    Build.PRODUCT.contains("sdk_google") ||
                    Build.PRODUCT.contains("vbox86p")
                ) {
                    newRating++
                }
                if (Build.MANUFACTURER == "unknown" || Build.MANUFACTURER == "Genymotion" ||
                    Build.MANUFACTURER.contains("Andy") ||
                    Build.MANUFACTURER.contains("MIT") ||
                    Build.MANUFACTURER.contains("nox") ||
                    Build.MANUFACTURER.contains("TiantianVM") ||
                    Build.MANUFACTURER.contains("vmos")
                ) {
                    newRating++
                }
                if (Build.BRAND == "generic" || Build.BRAND == "generic_x86" || Build.BRAND == "generic_x86_arm" || Build.BRAND == "TTVM" ||
                    Build.BRAND.contains("Andy")
                ) {
                    newRating++
                }
                if (Build.DEVICE.contains("generic") ||
                    Build.DEVICE.contains("generic_x86") || Build.DEVICE == "generic_x86_arm" ||
                    Build.DEVICE.contains("Andy") ||
                    Build.DEVICE.contains("ttVM_Hdragon") ||
                    Build.DEVICE.contains("Droid4X") ||
                    Build.DEVICE.contains("nox") ||
                    Build.DEVICE.contains("generic_x86_64") ||
                    Build.DEVICE.contains("sdk_gphone_x86_arm") ||
                    Build.DEVICE.contains("vbox86p")
                ) {
                    newRating++
                }
                if (Build.MODEL == "sdk" || Build.MODEL == "google_sdk" || Build.MODEL == "sdk_gphone_x86_arm" ||
                    Build.MODEL.contains("Droid4X") ||
                    Build.MODEL.contains("TiantianVM") ||
                    Build.MODEL.contains("Andy") || Build.MODEL == "Android SDK built for x86_64" || Build.MODEL == "Android SDK built for x86" || Build.MODEL == "vmos"
                ) {
                    newRating++
                }
                if (Build.HARDWARE == "goldfish" || Build.HARDWARE == "vbox86" ||
                    Build.HARDWARE.contains("nox") ||
                    Build.HARDWARE.contains("ttVM_x86")
                ) {
                    newRating++
                }
                if (Build.FINGERPRINT.contains("generic/sdk/generic") ||
                    Build.FINGERPRINT.contains("generic_x86/sdk_x86/generic_x86") ||
                    Build.FINGERPRINT.contains("sdk_gphone_x86_arm") ||
                    Build.FINGERPRINT.contains("Andy") ||
                    Build.FINGERPRINT.contains("ttVM_Hdragon") ||
                    Build.FINGERPRINT.contains("generic_x86_64") ||
                    Build.FINGERPRINT.contains("generic/google_sdk/generic") ||
                    Build.FINGERPRINT.contains("vbox86p") ||
                    Build.FINGERPRINT.contains("generic/vbox86p/vbox86p") ||
                    Build.FINGERPRINT.contains("test-keys")
                ) {
                    newRating++
                }
                try {
                    val opengl = GLES20.glGetString(GLES20.GL_RENDERER)
                    if (opengl != null) {
                        if (opengl.contains("Bluestacks") ||
                            opengl.contains("Translator")
                        ) newRating += 10
                    }
                } catch (e: Exception) {
                    e.printStackTrace()
                }
                try {
                    val sharedFolder = File(
                        Environment
                            .getExternalStorageDirectory().toString()
                                + File.separatorChar
                                + "windows"
                                + File.separatorChar
                                + "BstSharedFolder"
                    )
                    if (sharedFolder.exists()) {
                        newRating += 10
                    }
                } catch (e: Exception) {
                    e.printStackTrace()
                }
                rating = newRating
            }
            return rating > 3
        }

    fun getBytesFromBitmap(bitmap: Bitmap?): ByteArray {
        return nativeGetBytesFromBitmap(bitmap)
    }

    fun getByteBufferFromBitmap(bitmap: Bitmap?): ByteBuffer {
        return nativeGetByteBufferFromBitmap(bitmap)
    }


    @JvmStatic
    fun createSurfaceTexture(state: Long): Array<Any> {
        nativeMakeStateContextCurrent(state)
        val render = TextureRender()
        render.surfaceCreated()
        return arrayOf(SurfaceTexture(render.textureId), render)
    }


    @JvmStatic
    fun createRenderAndAttachToGLContext(
        state: Long,
        texture: SurfaceTexture,
    ): TextureRender {
        nativeMakeStateContextCurrent(state)
        val render = TextureRender()
        render.surfaceCreated()
        texture.attachToGLContext(render.textureId)
        return render
    }


    @JvmStatic
    fun attachToGLContext(
        state: Long,
        texture: SurfaceTexture,
        render: TextureRender
    ) {
        nativeMakeStateContextCurrent(state)
        texture.attachToGLContext(render.textureId)
    }

    @JvmStatic
    fun detachFromGLContext(state: Long, texture: SurfaceTexture) {
        nativeMakeStateContextCurrent(state)
        texture.detachFromGLContext()
    }

    @JvmStatic
    fun updateTexImage(
        state: Long,
        flipY: Boolean,
        texture: SurfaceTexture,
        render: TextureRender,
        width: Int,
        height: Int,
        internalFormat: Int,
        format: Int
    ) {
        nativeMakeStateContextCurrent(state)
        render.drawFrame(texture, width, height, internalFormat, format, flipY)

        if (render.width != width || render.height != width) {
            render.width = width
            render.height = height
        }
    }


}
