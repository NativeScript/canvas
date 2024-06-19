use jni::JNIEnv;
use once_cell::sync::OnceCell;

pub static SURFACE_TEXTURE: OnceCell<SurfaceTexture> = OnceCell::new();

type FromSurfaceTexture<'a> = Option<
    libloading::Symbol<
        'a,
        unsafe fn(
            env: *mut JNIEnv,
            surfacetexture: jni::sys::jobject,
        ) -> *mut super::surface_texture::ASurfaceTexture,
    >,
>;

type UpdateTexImage<'a> = Option<
    libloading::Symbol<
        'a,
        unsafe fn(st: *mut super::surface_texture::ASurfaceTexture) -> std::os::raw::c_int,
    >,
>;

type GetTransformMatrix<'a> = Option<
    libloading::Symbol<
        'a,
        unsafe fn(st: *mut super::surface_texture::ASurfaceTexture, mtx: *mut f32),
    >,
>;

type Release<'a> =
    Option<libloading::Symbol<'a, unsafe fn(st: *mut super::surface_texture::ASurfaceTexture)>>;

type FromSurfaceTextureRaw = Option<
    libloading::os::unix::Symbol<
        unsafe fn(
            env: *mut JNIEnv,
            surfacetexture: jni::sys::jobject,
        ) -> *mut super::surface_texture::ASurfaceTexture,
    >,
>;

type UpdateTexImageRaw = Option<
    libloading::os::unix::Symbol<
        unsafe fn(st: *mut super::surface_texture::ASurfaceTexture) -> std::os::raw::c_int,
    >,
>;

type GetTransformMatrixRaw = Option<
    libloading::os::unix::Symbol<
        unsafe fn(st: *mut super::surface_texture::ASurfaceTexture, mtx: *mut f32),
    >,
>;

type ReleaseRaw = Option<
    libloading::os::unix::Symbol<unsafe fn(st: *mut super::surface_texture::ASurfaceTexture)>,
>;

type FromSurfaceTextureRef<'a> = Option<
    &'a libloading::os::unix::Symbol<
        unsafe fn(
            env: *mut JNIEnv,
            surfacetexture: jni::sys::jobject,
        ) -> *mut super::surface_texture::ASurfaceTexture,
    >,
>;

type UpdateTexImageRef<'a> = Option<
    &'a libloading::os::unix::Symbol<
        unsafe fn(st: *mut super::surface_texture::ASurfaceTexture) -> std::os::raw::c_int,
    >,
>;

type GetTransformMatrixRef<'a> = Option<
    &'a libloading::os::unix::Symbol<
        unsafe fn(st: *mut super::surface_texture::ASurfaceTexture, mtx: *mut f32),
    >,
>;

type ReleaseRef<'a> = Option<
    &'a libloading::os::unix::Symbol<unsafe fn(st: *mut super::surface_texture::ASurfaceTexture)>,
>;

pub struct SurfaceTexture {
    lib: libloading::Library,
    from_surface_texture: FromSurfaceTextureRaw,
    update_tex_image: UpdateTexImageRaw,
    get_transform_matrix: GetTransformMatrixRaw,
    release: ReleaseRaw,
}

impl SurfaceTexture {
    pub fn new() -> Self {
        // should not fail to load the lib

        let lib = unsafe { libloading::Library::new("libandroid.so").unwrap() };

        let from_surface_texture: FromSurfaceTexture =
            unsafe { lib.get(b"ASurfaceTexture_fromSurfaceTexture\0").ok() };

        let from_surface_texture = from_surface_texture.map(|v| unsafe { v.into_raw() });

        let update_tex_image: UpdateTexImage =
            unsafe { lib.get(b"ASurfaceTexture_updateTexImage\0").ok() };

        let update_tex_image = update_tex_image.map(|v| unsafe { v.into_raw() });

        let get_transform_matrix: GetTransformMatrix =
            unsafe { lib.get(b"ASurfaceTexture_getTransformMatrix\0").ok() };

        let get_transform_matrix = get_transform_matrix.map(|v| unsafe { v.into_raw() });

        let release: Release = unsafe { lib.get(b"ASurfaceTexture_release\0").ok() };

        let release = release.map(|v| unsafe { v.into_raw() });

        Self {
            lib,
            from_surface_texture,
            update_tex_image,
            get_transform_matrix,
            release,
        }
    }

    pub fn from_surface_texture(&self) -> FromSurfaceTextureRef {
        self.from_surface_texture.as_ref()
    }
    pub fn update_tex_image(&self) -> UpdateTexImageRef {
        self.update_tex_image.as_ref()
    }
    pub fn get_transform_matrix(&self) -> GetTransformMatrixRef {
        self.get_transform_matrix.as_ref()
    }
    pub fn release(&self) -> ReleaseRef {
        self.release.as_ref()
    }
}

impl Drop for SurfaceTexture {
    fn drop(&mut self) {
        if let Some(from_surface_texture) = self.from_surface_texture.clone() {
            let _ = unsafe { libloading::Symbol::from_raw(from_surface_texture, &self.lib) };
        }
        if let Some(update_tex_image) = self.update_tex_image.clone() {
            let _ = unsafe { libloading::Symbol::from_raw(update_tex_image, &self.lib) };
        }
        if let Some(get_transform_matrix) = self.get_transform_matrix.clone() {
            let _ = unsafe { libloading::Symbol::from_raw(get_transform_matrix, &self.lib) };
        }
        if let Some(release) = self.release.clone() {
            let _ = unsafe { libloading::Symbol::from_raw(release, &self.lib) };
        }
    }
}
