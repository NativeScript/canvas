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

    /** Makes the EGL context of a [CanvasRenderingContext2D] current on the calling thread.
     *  [context] is the native pointer returned by [NSCCanvas.nativeContext]. */
    @JvmStatic
    private external fun nativeMake2DContextCurrent(context: Long): Boolean

    /** Draw an OES external texture into a 2D canvas context using Skia's GPU path.
     *  The EGL context must already be current (call [nativeMake2DContextCurrent] first
     *  and call [SurfaceTexture.updateTexImage] before calling this function). */
    @JvmStatic
    private external fun nativeContext2DDrawOESTexture(
        context: Long,
        textureId: Int,
        videoWidth: Int,
        videoHeight: Int,
        sx: Float,
        sy: Float,
        sw: Float,
        sh: Float,
        dx: Float,
        dy: Float,
        dw: Float,
        dh: Float,
    ): Boolean

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

    /**
     * Create an OES texture + [SurfaceTexture] inside the 2D canvas context's EGL context.
     *
     * The player should be directed to output frames to the returned [SurfaceTexture].
     * After a frame arrives, call [drawVideoFrameGL] to upload and draw it via Skia's
     * GL fast-path (zero CPU copy).
     *
     * @param context Native pointer to `CanvasRenderingContext2D` (from `NSCCanvas.nativeContext`).
     * @return `[SurfaceTexture, Integer textureId, TextureRender]`, or `null` if the context
     *         has no GL backend.
     */
    @JvmStatic
    fun create2DContextSurfaceTexture(context: Long): Array<Any>? {
        if (!nativeMake2DContextCurrent(context)) return null
        val render = TextureRender()
        render.surfaceCreated()
        return arrayOf(SurfaceTexture(render.textureId), render.textureId, render)
    }

    /**
     * Draw the latest video frame from [surfaceTexture] into a 2D canvas context via
     * Skia's GL fast-path (OES texture → Skia GPU image, zero CPU copy).
     *
     * Call this from whatever thread owns the 2D canvas context.  The function makes
     * the context current, calls [SurfaceTexture.updateTexImage], then asks Rust/Skia
     * to draw the OES texture directly.
     *
     * @param context     Native pointer to `CanvasRenderingContext2D`.
     * @param textureId   OES texture name that backs [surfaceTexture].
     * @param surfaceTexture The SurfaceTexture receiving decoded video frames.
     * @param videoWidth  / [videoHeight] Video frame dimensions.
     * @param sx,sy,sw,sh Source rectangle within the video frame.
     * @param dx,dy,dw,dh Destination rectangle in canvas coordinates.
     * @return `true` when the frame was drawn successfully.
     */
    @JvmStatic
    fun drawVideoFrameGL(
        context: Long,
        textureId: Int,
        surfaceTexture: SurfaceTexture,
        videoWidth: Int,
        videoHeight: Int,
        sx: Float,
        sy: Float,
        sw: Float,
        sh: Float,
        dx: Float,
        dy: Float,
        dw: Float,
        dh: Float,
    ): Boolean {
        if (!nativeMake2DContextCurrent(context)) return false
        surfaceTexture.updateTexImage()
        return nativeContext2DDrawOESTexture(
            context, textureId,
            videoWidth, videoHeight,
            sx, sy, sw, sh,
            dx, dy, dw, dh,
        )
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
