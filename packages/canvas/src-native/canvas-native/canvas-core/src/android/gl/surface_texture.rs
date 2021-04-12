#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ANativeWindow {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ASurfaceTexture {
    _unused: [u8; 0],
}



extern "system" {
    #[doc = " Release the reference to the native ASurfaceTexture acquired with"]
    #[doc = " ASurfaceTexture_fromSurfaceTexture()."]
    #[doc = " Failing to do so will result in leaked memory and graphic resources."]
    #[doc = ""]
    #[doc = " Available since API level 28."]
    #[doc = ""]
    #[doc = " \\param st A ASurfaceTexture reference acquired with ASurfaceTexture_fromSurfaceTexture()"]
    pub fn ASurfaceTexture_release(st: *mut ASurfaceTexture);
}
extern "system" {
    #[doc = " Returns a reference to an ANativeWindow (i.e. the Producer) for this SurfaceTexture."]
    #[doc = " This is equivalent to Java's: Surface sur = new Surface(surfaceTexture);"]
    #[doc = ""]
    #[doc = " Available since API level 28."]
    #[doc = ""]
    #[doc = " \\param st A ASurfaceTexture reference acquired with ASurfaceTexture_fromSurfaceTexture()"]
    #[doc = " @return A reference to an ANativeWindow. This reference MUST BE released when no longer needed"]
    #[doc = " using ANativeWindow_release(). Failing to do so will result in leaked resources. nullptr is"]
    #[doc = " returned if \\p st is null or if it's not an instance of android.graphics.SurfaceTexture"]
    pub fn ASurfaceTexture_acquireANativeWindow(st: *mut ASurfaceTexture) -> *mut ANativeWindow;
}
extern "system" {
    #[doc = " Attach the SurfaceTexture to the OpenGL ES context that is current on the calling thread.  A"]
    #[doc = " new OpenGL ES texture object is created and populated with the SurfaceTexture image frame"]
    #[doc = " that was current at the time of the last call to {@link #detachFromGLContext}.  This new"]
    #[doc = " texture is bound to the GL_TEXTURE_EXTERNAL_OES texture target."]
    #[doc = ""]
    #[doc = " This can be used to access the SurfaceTexture image contents from multiple OpenGL ES"]
    #[doc = " contexts.  Note, however, that the image contents are only accessible from one OpenGL ES"]
    #[doc = " context at a time."]
    #[doc = ""]
    #[doc = " Available since API level 28."]
    #[doc = ""]
    #[doc = " \\param st A ASurfaceTexture reference acquired with ASurfaceTexture_fromSurfaceTexture()"]
    #[doc = " \\param texName The name of the OpenGL ES texture that will be created.  This texture name"]
    #[doc = " must be unusued in the OpenGL ES context that is current on the calling thread."]
    #[doc = " \\return 0 on success, negative posix error code otherwise (see <errno.h>)"]
    pub fn ASurfaceTexture_attachToGLContext(
        st: *mut ASurfaceTexture,
        texName: u32,
    ) -> ::std::os::raw::c_int;
}
extern "system" {
    #[doc = " Detach the SurfaceTexture from the OpenGL ES context that owns the OpenGL ES texture object."]
    #[doc = " This call must be made with the OpenGL ES context current on the calling thread.  The OpenGL"]
    #[doc = " ES texture object will be deleted as a result of this call.  After calling this method all"]
    #[doc = " calls to {@link #updateTexImage} will fail until a successful call to {@link #attachToGLContext}"]
    #[doc = " is made."]
    #[doc = ""]
    #[doc = " This can be used to access the SurfaceTexture image contents from multiple OpenGL ES"]
    #[doc = " contexts.  Note, however, that the image contents are only accessible from one OpenGL ES"]
    #[doc = " context at a time."]
    #[doc = ""]
    #[doc = " Available since API level 28."]
    #[doc = ""]
    #[doc = " \\param st A ASurfaceTexture reference acquired with ASurfaceTexture_fromSurfaceTexture()"]
    #[doc = " \\return 0 on success, negative posix error code otherwise (see <errno.h>)"]
    pub fn ASurfaceTexture_detachFromGLContext(st: *mut ASurfaceTexture) -> ::std::os::raw::c_int;
}
extern "system" {
    #[doc = " Update the texture image to the most recent frame from the image stream.  This may only be"]
    #[doc = " called while the OpenGL ES context that owns the texture is current on the calling thread."]
    #[doc = " It will implicitly bind its texture to the GL_TEXTURE_EXTERNAL_OES texture target."]
    #[doc = ""]
    #[doc = " Available since API level 28."]
    #[doc = ""]
    #[doc = " \\param st A ASurfaceTexture reference acquired with ASurfaceTexture_fromSurfaceTexture()"]
    #[doc = " \\return 0 on success, negative posix error code otherwise (see <errno.h>)"]
    pub fn ASurfaceTexture_updateTexImage(st: *mut ASurfaceTexture) -> ::std::os::raw::c_int;
}
extern "system" {
    #[doc = " Retrieve the 4x4 texture coordinate transform matrix associated with the texture image set by"]
    #[doc = " the most recent call to updateTexImage."]
    #[doc = ""]
    #[doc = " This transform matrix maps 2D homogeneous texture coordinates of the form (s, t, 0, 1) with s"]
    #[doc = " and t in the inclusive range [0, 1] to the texture coordinate that should be used to sample"]
    #[doc = " that location from the texture.  Sampling the texture outside of the range of this transform"]
    #[doc = " is undefined."]
    #[doc = ""]
    #[doc = " The matrix is stored in column-major order so that it may be passed directly to OpenGL ES via"]
    #[doc = " the glLoadMatrixf or glUniformMatrix4fv functions."]
    #[doc = ""]
    #[doc = " Available since API level 28."]
    #[doc = ""]
    #[doc = " \\param st A ASurfaceTexture reference acquired with ASurfaceTexture_fromSurfaceTexture()"]
    #[doc = " \\param mtx the array into which the 4x4 matrix will be stored.  The array must have exactly"]
    #[doc = "     16 elements."]
    pub fn ASurfaceTexture_getTransformMatrix(st: *mut ASurfaceTexture, mtx: *mut f32);
}
extern "system" {
    #[doc = " Retrieve the timestamp associated with the texture image set by the most recent call to"]
    #[doc = " updateTexImage."]
    #[doc = ""]
    #[doc = " This timestamp is in nanoseconds, and is normally monotonically increasing. The timestamp"]
    #[doc = " should be unaffected by time-of-day adjustments, and for a camera should be strictly"]
    #[doc = " monotonic but for a MediaPlayer may be reset when the position is set.  The"]
    #[doc = " specific meaning and zero point of the timestamp depends on the source providing images to"]
    #[doc = " the SurfaceTexture. Unless otherwise specified by the image source, timestamps cannot"]
    #[doc = " generally be compared across SurfaceTexture instances, or across multiple program"]
    #[doc = " invocations. It is mostly useful for determining time offsets between subsequent frames."]
    #[doc = ""]
    #[doc = " For EGL/Vulkan producers, this timestamp is the desired present time set with the"]
    #[doc = " EGL_ANDROID_presentation_time or VK_GOOGLE_display_timing extensions"]
    #[doc = ""]
    #[doc = " Available since API level 28."]
    #[doc = ""]
    #[doc = " \\param st A ASurfaceTexture reference acquired with ASurfaceTexture_fromSurfaceTexture()"]
    pub fn ASurfaceTexture_getTimestamp(st: *mut ASurfaceTexture) -> i64;
}