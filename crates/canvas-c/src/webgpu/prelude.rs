use std::{ffi::CStr, os::raw::c_char};

pub fn build_features(features: wgpu::Features) -> Vec<&'static str> {
    let mut return_features: Vec<&'static str> = vec![];

    // api
    if features.contains(wgpu::Features::DEPTH_CLIP_CONTROL) {
        return_features.push("depth-clip-control");
    }
    if features.contains(wgpu::Features::TIMESTAMP_QUERY) {
        return_features.push("timestamp-query");
    }
    if features.contains(wgpu::Features::INDIRECT_FIRST_INSTANCE) {
        return_features.push("indirect-first-instance");
    }
    // shader
    if features.contains(wgpu::Features::SHADER_F16) {
        return_features.push("shader-f16");
    }
    // texture formats
    if features.contains(wgpu::Features::DEPTH32FLOAT_STENCIL8) {
        return_features.push("depth32float-stencil8");
    }
    if features.contains(wgpu::Features::TEXTURE_COMPRESSION_BC) {
        return_features.push("texture-compression-bc");
    }
    if features.contains(wgpu::Features::TEXTURE_COMPRESSION_ETC2) {
        return_features.push("texture-compression-etc2");
    }
    if features.contains(wgpu::Features::TEXTURE_COMPRESSION_ASTC) {
        return_features.push("texture-compression-astc");
    }
    if features.contains(wgpu::Features::RG11B10UFLOAT_RENDERABLE) {
        return_features.push("rg11b10ufloat-renderable");
    }
    if features.contains(wgpu::Features::BGRA8UNORM_STORAGE) {
        return_features.push("bgra8unorm-storage");
    }
    if features.contains(wgpu::Features::FLOAT32_FILTERABLE) {
        return_features.push("float32-filterable");
    }

    // extended from spec

    // texture formats
    if features.contains(wgpu::Features::TEXTURE_FORMAT_16BIT_NORM) {
        return_features.push("texture-format-16-bit-norm");
    }
    if features.contains(wgpu::Features::TEXTURE_COMPRESSION_ASTC_HDR) {
        return_features.push("texture-compression-astc-hdr");
    }
    if features.contains(wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES) {
        return_features.push("texture-adapter-specific-format-features");
    }
    // api
    if features.contains(wgpu::Features::PIPELINE_STATISTICS_QUERY) {
        return_features.push("pipeline-statistics-query");
    }
    if features.contains(wgpu::Features::TIMESTAMP_QUERY_INSIDE_PASSES) {
        return_features.push("timestamp-query-inside-passes");
    }
    if features.contains(wgpu::Features::MAPPABLE_PRIMARY_BUFFERS) {
        return_features.push("mappable-primary-buffers");
    }
    if features.contains(wgpu::Features::TEXTURE_BINDING_ARRAY) {
        return_features.push("texture-binding-array");
    }
    if features.contains(wgpu::Features::BUFFER_BINDING_ARRAY) {
        return_features.push("buffer-binding-array");
    }
    if features.contains(wgpu::Features::STORAGE_RESOURCE_BINDING_ARRAY) {
        return_features.push("storage-resource-binding-array");
    }
    if features
        .contains(wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING)
    {
        return_features.push("sampled-texture-and-storage-buffer-array-non-uniform-indexing");
    }
    if features
        .contains(wgpu::Features::UNIFORM_BUFFER_AND_STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING)
    {
        return_features.push("uniform-buffer-and-storage-texture-array-non-uniform-indexing");
    }
    if features.contains(wgpu::Features::PARTIALLY_BOUND_BINDING_ARRAY) {
        return_features.push("partially-bound-binding-array");
    }
    if features.contains(wgpu::Features::MULTI_DRAW_INDIRECT) {
        return_features.push("multi-draw-indirect");
    }
    if features.contains(wgpu::Features::MULTI_DRAW_INDIRECT_COUNT) {
        return_features.push("multi-draw-indirect-count");
    }
    if features.contains(wgpu::Features::PUSH_CONSTANTS) {
        return_features.push("push-constants");
    }
    if features.contains(wgpu::Features::ADDRESS_MODE_CLAMP_TO_ZERO) {
        return_features.push("address-mode-clamp-to-zero");
    }
    if features.contains(wgpu::Features::ADDRESS_MODE_CLAMP_TO_BORDER) {
        return_features.push("address-mode-clamp-to-border");
    }
    if features.contains(wgpu::Features::POLYGON_MODE_LINE) {
        return_features.push("polygon-mode-line");
    }
    if features.contains(wgpu::Features::POLYGON_MODE_POINT) {
        return_features.push("polygon-mode-point");
    }
    if features.contains(wgpu::Features::CONSERVATIVE_RASTERIZATION) {
        return_features.push("conservative-rasterization");
    }
    if features.contains(wgpu::Features::VERTEX_WRITABLE_STORAGE) {
        return_features.push("vertex-writable-storage");
    }
    if features.contains(wgpu::Features::CLEAR_TEXTURE) {
        return_features.push("clear-texture");
    }
    if features.contains(wgpu::Features::SPIRV_SHADER_PASSTHROUGH) {
        return_features.push("spirv-shader-passthrough");
    }
    if features.contains(wgpu::Features::MULTIVIEW) {
        return_features.push("multiview");
    }
    if features.contains(wgpu::Features::VERTEX_ATTRIBUTE_64BIT) {
        return_features.push("vertex-attribute-64-bit");
    }
    // shader
    if features.contains(wgpu::Features::SHADER_F64) {
        return_features.push("shader-f64");
    }
    if features.contains(wgpu::Features::SHADER_I16) {
        return_features.push("shader-i16");
    }
    if features.contains(wgpu::Features::SHADER_PRIMITIVE_INDEX) {
        return_features.push("shader-primitive-index");
    }
    if features.contains(wgpu::Features::SHADER_EARLY_DEPTH_TEST) {
        return_features.push("shader-early-depth-test");
    }
    if features.contains(wgpu::Features::SHADER_UNUSED_VERTEX_OUTPUT) {
        return_features.push("shader-unused-vertex-output");
    }

    return_features
}

pub fn parse_required_features(
    required_features: *const *const c_char,
    required_features_length: usize,
) -> wgpu::Features {
    let mut features = wgpu::Features::empty();

    if required_features.is_null() && required_features_length > 0 {
        let feats =
            unsafe { std::slice::from_raw_parts(required_features, required_features_length) };
        for feat in feats {
            let feat = unsafe { CStr::from_ptr(*feat) };
            let feat = feat.to_string_lossy();
            let feat = feat.as_ref();
            match feat {
                "depth-clip-control" => {
                    features.set(wgpu::Features::DEPTH_CLIP_CONTROL, true);
                }

                "timestamp-query" => {
                    features.set(wgpu::Features::TIMESTAMP_QUERY, true);
                }
                "indirect-first-instance" => {
                    features.set(wgpu::Features::INDIRECT_FIRST_INSTANCE, true);
                }

                "shader-f16" => {
                    features.set(wgpu::Features::SHADER_F16, true);
                }
                "depth32float-stencil8" => {
                    features.set(wgpu::Features::DEPTH32FLOAT_STENCIL8, true);
                }
                "texture-compression-bc" => {
                    features.set(wgpu::Features::TEXTURE_COMPRESSION_BC, true);
                }
                "texture-compression-etc2" => {
                    features.set(wgpu::Features::TEXTURE_COMPRESSION_ETC2, true);
                }

                "texture-compression-astc" => {
                    features.set(wgpu::Features::TEXTURE_COMPRESSION_ASTC, true);
                }

                "g11b10ufloat-renderable" => {
                    features.set(wgpu::Features::RG11B10UFLOAT_RENDERABLE, true);
                }
                "bgra8unorm-storage" => {
                    features.set(wgpu::Features::BGRA8UNORM_STORAGE, true);
                }

                "float32-filterable" => {
                    features.set(wgpu::Features::FLOAT32_FILTERABLE, true);
                }

                "texture-format-16-bit-norm" => {
                    features.set(wgpu::Features::TEXTURE_FORMAT_16BIT_NORM, true);
                }

                "texture-compression-astc-hdr" => {
                    features.set(wgpu::Features::TEXTURE_COMPRESSION_ASTC_HDR, true);
                }

                "texture-adapter-specific-format-features" => {
                    features.set(
                        wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                        true,
                    );
                }

                "pipeline-statistics-query" => {
                    features.set(wgpu::Features::PIPELINE_STATISTICS_QUERY, true);
                }
                "timestamp-query-inside-passes" => {
                    features.set(wgpu::Features::TIMESTAMP_QUERY_INSIDE_PASSES, true);
                }

                "mappable-primary-buffers" => {
                    features.set(wgpu::Features::MAPPABLE_PRIMARY_BUFFERS, true);
                }

                "texture-binding-array" => {
                    features.set(wgpu::Features::TEXTURE_BINDING_ARRAY, true);
                }

                "buffer-binding-array" => {
                    features.set(wgpu::Features::BUFFER_BINDING_ARRAY, true);
                }

                "storage-resource-binding-array" => {
                    features.set(wgpu::Features::STORAGE_RESOURCE_BINDING_ARRAY, true);
                }

                "sampled-texture-and-storage-buffer-array-non-uniform-indexing" => {
                    features.set(
                            wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING,
                            true,
                        );
                }

                "uniform-buffer-and-storage-texture-array-non-uniform-indexing" => {
                    features.set(
                            wgpu::Features::UNIFORM_BUFFER_AND_STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING,
                            true,
                        );
                }

                "partially-bound-binding-array" => {
                    features.set(wgpu::Features::PARTIALLY_BOUND_BINDING_ARRAY, true);
                }

                "multi-draw-indirect" => {
                    features.set(wgpu::Features::MULTI_DRAW_INDIRECT, true);
                }

                "multi-draw-indirect-count" => {
                    features.set(wgpu::Features::MULTI_DRAW_INDIRECT_COUNT, true);
                }

                "push-constants" => {
                    features.set(wgpu::Features::PUSH_CONSTANTS, true);
                }

                "address-mode-clamp-to-zero" => {
                    features.set(wgpu::Features::ADDRESS_MODE_CLAMP_TO_ZERO, true);
                }

                "address-mode-clamp-to-border" => {
                    features.set(wgpu::Features::ADDRESS_MODE_CLAMP_TO_BORDER, true);
                }

                "polygon-mode-line" => {
                    features.set(wgpu::Features::POLYGON_MODE_LINE, true);
                }

                "ppolygon-mode-point" => {
                    features.set(wgpu::Features::POLYGON_MODE_POINT, true);
                }

                "conservative-rasterization" => {
                    features.set(wgpu::Features::CONSERVATIVE_RASTERIZATION, true);
                }

                "vertex-writable-storage" => {
                    features.set(wgpu::Features::VERTEX_WRITABLE_STORAGE, true);
                }

                "clear-texture" => {
                    features.set(wgpu::Features::CLEAR_TEXTURE, true);
                }

                "pirv-shader-passthrough" => {
                    features.set(wgpu::Features::SPIRV_SHADER_PASSTHROUGH, true);
                }

                "multiview" => {
                    features.set(wgpu::Features::MULTIVIEW, true);
                }

                "vertex-attribute-64-bit" => {
                    features.set(wgpu::Features::VERTEX_ATTRIBUTE_64BIT, true);
                }

                "shader-f64" => {
                    features.set(wgpu::Features::SHADER_F64, true);
                }

                "shader-i16" => {
                    features.set(wgpu::Features::SHADER_I16, true);
                }

                "shader-primitive-index" => {
                    features.set(wgpu::Features::SHADER_PRIMITIVE_INDEX, true);
                }

                "shader-early-depth-test" => {
                    features.set(wgpu::Features::SHADER_EARLY_DEPTH_TEST, true);
                }

                "shader-unused-vertex-output" => {
                    features.set(wgpu::Features::SHADER_UNUSED_VERTEX_OUTPUT, true);
                }
                _ => {}
            }
        }
    }

    features
}
