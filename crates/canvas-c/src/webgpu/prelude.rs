use std::{ffi::CStr, os::raw::c_char};
use std::borrow::Cow;

#[inline]
pub(crate) fn ptr_into_label<'a>(ptr: *const std::ffi::c_char) -> wgpu_core::Label<'a> {
    unsafe { ptr.as_ref() }.and_then(|ptr| {
        unsafe { CStr::from_ptr(ptr) }
            .to_str()
            .ok()
            .map(Cow::Borrowed)
    })
}

pub fn build_features(features: wgt::Features) -> Vec<&'static str> {
    let mut return_features: Vec<&'static str> = vec![];

    // api
    if features.contains(wgt::Features::DEPTH_CLIP_CONTROL) {
        return_features.push("depth-clip-control");
    }
    if features.contains(wgt::Features::TIMESTAMP_QUERY) {
        return_features.push("timestamp-query");
    }
    if features.contains(wgt::Features::INDIRECT_FIRST_INSTANCE) {
        return_features.push("indirect-first-instance");
    }
    // shader
    if features.contains(wgt::Features::SHADER_F16) {
        return_features.push("shader-f16");
    }
    // texture formats
    if features.contains(wgt::Features::DEPTH32FLOAT_STENCIL8) {
        return_features.push("depth32float-stencil8");
    }
    if features.contains(wgt::Features::TEXTURE_COMPRESSION_BC) {
        return_features.push("texture-compression-bc");
    }
    if features.contains(wgt::Features::TEXTURE_COMPRESSION_ETC2) {
        return_features.push("texture-compression-etc2");
    }
    if features.contains(wgt::Features::TEXTURE_COMPRESSION_ASTC) {
        return_features.push("texture-compression-astc");
    }
    if features.contains(wgt::Features::RG11B10UFLOAT_RENDERABLE) {
        return_features.push("rg11b10ufloat-renderable");
    }
    if features.contains(wgt::Features::BGRA8UNORM_STORAGE) {
        return_features.push("bgra8unorm-storage");
    }
    if features.contains(wgt::Features::FLOAT32_FILTERABLE) {
        return_features.push("float32-filterable");
    }
    if features.contains(wgt::Features::DUAL_SOURCE_BLENDING) {
        return_features.push("dual-source-blending");
    }

    // extended from spec

    // texture formats
    if features.contains(wgt::Features::TEXTURE_FORMAT_16BIT_NORM) {
        return_features.push("texture-format-16-bit-norm");
    }
    if features.contains(wgt::Features::TEXTURE_COMPRESSION_ASTC_HDR) {
        return_features.push("texture-compression-astc-hdr");
    }
    if features.contains(wgt::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES) {
        return_features.push("texture-adapter-specific-format-features");
    }
    // api
    if features.contains(wgt::Features::PIPELINE_STATISTICS_QUERY) {
        return_features.push("pipeline-statistics-query");
    }
    if features.contains(wgt::Features::TIMESTAMP_QUERY_INSIDE_PASSES) {
        return_features.push("timestamp-query-inside-passes");
    }
    if features.contains(wgt::Features::MAPPABLE_PRIMARY_BUFFERS) {
        return_features.push("mappable-primary-buffers");
    }
    if features.contains(wgt::Features::TEXTURE_BINDING_ARRAY) {
        return_features.push("texture-binding-array");
    }
    if features.contains(wgt::Features::BUFFER_BINDING_ARRAY) {
        return_features.push("buffer-binding-array");
    }
    if features.contains(wgt::Features::STORAGE_RESOURCE_BINDING_ARRAY) {
        return_features.push("storage-resource-binding-array");
    }
    if features.contains(
        wgt::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING,
    ) {
        return_features.push("sampled-texture-and-storage-buffer-array-non-uniform-indexing");
    }
    if features.contains(
        wgt::Features::UNIFORM_BUFFER_AND_STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING,
    ) {
        return_features.push("uniform-buffer-and-storage-texture-array-non-uniform-indexing");
    }
    if features.contains(wgt::Features::PARTIALLY_BOUND_BINDING_ARRAY) {
        return_features.push("partially-bound-binding-array");
    }
    if features.contains(wgt::Features::MULTI_DRAW_INDIRECT) {
        return_features.push("multi-draw-indirect");
    }
    if features.contains(wgt::Features::MULTI_DRAW_INDIRECT_COUNT) {
        return_features.push("multi-draw-indirect-count");
    }
    if features.contains(wgt::Features::PUSH_CONSTANTS) {
        return_features.push("push-constants");
    }
    if features.contains(wgt::Features::ADDRESS_MODE_CLAMP_TO_ZERO) {
        return_features.push("address-mode-clamp-to-zero");
    }
    if features.contains(wgt::Features::ADDRESS_MODE_CLAMP_TO_BORDER) {
        return_features.push("address-mode-clamp-to-border");
    }
    if features.contains(wgt::Features::POLYGON_MODE_LINE) {
        return_features.push("polygon-mode-line");
    }
    if features.contains(wgt::Features::POLYGON_MODE_POINT) {
        return_features.push("polygon-mode-point");
    }
    if features.contains(wgt::Features::CONSERVATIVE_RASTERIZATION) {
        return_features.push("conservative-rasterization");
    }
    if features.contains(wgt::Features::VERTEX_WRITABLE_STORAGE) {
        return_features.push("vertex-writable-storage");
    }
    if features.contains(wgt::Features::CLEAR_TEXTURE) {
        return_features.push("clear-texture");
    }
    if features.contains(wgt::Features::SPIRV_SHADER_PASSTHROUGH) {
        return_features.push("spirv-shader-passthrough");
    }
    if features.contains(wgt::Features::MULTIVIEW) {
        return_features.push("multiview");
    }
    if features.contains(wgt::Features::VERTEX_ATTRIBUTE_64BIT) {
        return_features.push("vertex-attribute-64-bit");
    }
    // shader
    if features.contains(wgt::Features::SHADER_F64) {
        return_features.push("shader-f64");
    }
    if features.contains(wgt::Features::SHADER_I16) {
        return_features.push("shader-i16");
    }
    if features.contains(wgt::Features::SHADER_PRIMITIVE_INDEX) {
        return_features.push("shader-primitive-index");
    }
    if features.contains(wgt::Features::SHADER_EARLY_DEPTH_TEST) {
        return_features.push("shader-early-depth-test");
    }

    return_features
}

pub fn parse_required_features(
    required_features: *const *const c_char,
    required_features_length: usize,
) -> wgt::Features {
    let mut features = wgt::Features::empty();

    if !required_features.is_null() && required_features_length > 0 {
        let feats =
            unsafe { std::slice::from_raw_parts(required_features, required_features_length) };
        for feat in feats {
            let feat = unsafe { CStr::from_ptr(*feat) };
            let feat = feat.to_string_lossy();
            let feat = feat.as_ref();
            match feat {
                "depth-clip-control" => {
                    features.set(wgt::Features::DEPTH_CLIP_CONTROL, true);
                }

                "timestamp-query" => {
                    features.set(wgt::Features::TIMESTAMP_QUERY, true);
                }
                "indirect-first-instance" => {
                    features.set(wgt::Features::INDIRECT_FIRST_INSTANCE, true);
                }

                "shader-f16" => {
                    features.set(wgt::Features::SHADER_F16, true);
                }
                "depth32float-stencil8" => {
                    features.set(wgt::Features::DEPTH32FLOAT_STENCIL8, true);
                }
                "texture-compression-bc" => {
                    features.set(wgt::Features::TEXTURE_COMPRESSION_BC, true);
                }
                "texture-compression-etc2" => {
                    features.set(wgt::Features::TEXTURE_COMPRESSION_ETC2, true);
                }

                "texture-compression-astc" => {
                    features.set(wgt::Features::TEXTURE_COMPRESSION_ASTC, true);
                }

                "g11b10ufloat-renderable" => {
                    features.set(wgt::Features::RG11B10UFLOAT_RENDERABLE, true);
                }
                "bgra8unorm-storage" => {
                    features.set(wgt::Features::BGRA8UNORM_STORAGE, true);
                }

                "float32-filterable" => {
                    features.set(wgt::Features::FLOAT32_FILTERABLE, true);
                }

                "texture-format-16-bit-norm" => {
                    features.set(wgt::Features::TEXTURE_FORMAT_16BIT_NORM, true);
                }

                "texture-compression-astc-hdr" => {
                    features.set(wgt::Features::TEXTURE_COMPRESSION_ASTC_HDR, true);
                }

                "texture-adapter-specific-format-features" => {
                    features.set(
                        wgt::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                        true,
                    );
                }

                "pipeline-statistics-query" => {
                    features.set(wgt::Features::PIPELINE_STATISTICS_QUERY, true);
                }
                "timestamp-query-inside-passes" => {
                    features.set(wgt::Features::TIMESTAMP_QUERY_INSIDE_PASSES, true);
                }

                "mappable-primary-buffers" => {
                    features.set(wgt::Features::MAPPABLE_PRIMARY_BUFFERS, true);
                }

                "texture-binding-array" => {
                    features.set(wgt::Features::TEXTURE_BINDING_ARRAY, true);
                }

                "buffer-binding-array" => {
                    features.set(wgt::Features::BUFFER_BINDING_ARRAY, true);
                }

                "storage-resource-binding-array" => {
                    features.set(wgt::Features::STORAGE_RESOURCE_BINDING_ARRAY, true);
                }

                "sampled-texture-and-storage-buffer-array-non-uniform-indexing" => {
                    features.set(
                        wgt::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING,
                        true,
                    );
                }

                "uniform-buffer-and-storage-texture-array-non-uniform-indexing" => {
                    features.set(
                        wgt::Features::UNIFORM_BUFFER_AND_STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING,
                        true,
                    );
                }

                "partially-bound-binding-array" => {
                    features.set(wgt::Features::PARTIALLY_BOUND_BINDING_ARRAY, true);
                }

                "multi-draw-indirect" => {
                    features.set(wgt::Features::MULTI_DRAW_INDIRECT, true);
                }

                "multi-draw-indirect-count" => {
                    features.set(wgt::Features::MULTI_DRAW_INDIRECT_COUNT, true);
                }

                "push-constants" => {
                    features.set(wgt::Features::PUSH_CONSTANTS, true);
                }

                "address-mode-clamp-to-zero" => {
                    features.set(wgt::Features::ADDRESS_MODE_CLAMP_TO_ZERO, true);
                }

                "address-mode-clamp-to-border" => {
                    features.set(wgt::Features::ADDRESS_MODE_CLAMP_TO_BORDER, true);
                }

                "polygon-mode-line" => {
                    features.set(wgt::Features::POLYGON_MODE_LINE, true);
                }

                "ppolygon-mode-point" => {
                    features.set(wgt::Features::POLYGON_MODE_POINT, true);
                }

                "conservative-rasterization" => {
                    features.set(wgt::Features::CONSERVATIVE_RASTERIZATION, true);
                }

                "vertex-writable-storage" => {
                    features.set(wgt::Features::VERTEX_WRITABLE_STORAGE, true);
                }

                "clear-texture" => {
                    features.set(wgt::Features::CLEAR_TEXTURE, true);
                }

                "pirv-shader-passthrough" => {
                    features.set(wgt::Features::SPIRV_SHADER_PASSTHROUGH, true);
                }

                "multiview" => {
                    features.set(wgt::Features::MULTIVIEW, true);
                }

                "vertex-attribute-64-bit" => {
                    features.set(wgt::Features::VERTEX_ATTRIBUTE_64BIT, true);
                }

                "shader-f64" => {
                    features.set(wgt::Features::SHADER_F64, true);
                }

                "shader-i16" => {
                    features.set(wgt::Features::SHADER_I16, true);
                }

                "shader-primitive-index" => {
                    features.set(wgt::Features::SHADER_PRIMITIVE_INDEX, true);
                }

                "shader-early-depth-test" => {
                    features.set(wgt::Features::SHADER_EARLY_DEPTH_TEST, true);
                }

                "dual-source-blending" => {
                    features.set(
                        wgt::Features::DUAL_SOURCE_BLENDING, true);
                }

                _ => {}
            }
        }
    }

    features
}
