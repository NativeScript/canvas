cmake_minimum_required(VERSION 3.15.0)
set(CMAKE_VERBOSE_MAKEFILE ON)

project(canvasnativev8)

set(CMAKE_CXX_STANDARD 17)


option(ENABLE_LTO "Enable cross language linking time optimization" ON)
if (ENABLE_LTO)
  include(CheckIPOSupported)
  check_ipo_supported(RESULT supported OUTPUT error)
  if (supported)
    message(STATUS "IPO / LTO enabled")
    set(CMAKE_INTERPROCEDURAL_OPTIMIZATION TRUE)
    add_link_options(-fuse-ld=lld)
  else ()
    message(STATUS "IPO / LTO not supported: <${error}>")
  endif ()
endif ()


if ("${ANDROID_ABI}" MATCHES "armeabi-v7a$")
  set(RUST_BUILD_TARGET "armv7-linux-androideabi")
elseif ("${ANDROID_ABI}" MATCHES "arm64-v8a$")
  set(RUST_BUILD_TARGET "aarch64-linux-android")
elseif ("${ANDROID_ABI}" MATCHES "x86_64$")
  set(RUST_BUILD_TARGET "x86_64-linux-android")
elseif ("${ANDROID_ABI}" MATCHES "x86$")
  set(RUST_BUILD_TARGET "i686-linux-android")
endif ()

get_filename_component(PROJECT_ROOT_DIR ${PROJECT_SOURCE_DIR} DIRECTORY)

get_filename_component(PROJECT_NATIVE_ROOT_DIR ${PROJECT_ROOT_DIR} DIRECTORY)

set(PROJECT_NATIVE_ROOT_DIR ${PROJECT_NATIVE_ROOT_DIR}/canvas-native)


set(CARGO_MANIFEST ${PROJECT_NATIVE_ROOT_DIR}/Cargo.toml)
set(CARGO_TARGET_DIR ${PROJECT_NATIVE_ROOT_DIR}/target)
set(CURRENT_TARGET_DIR ${CARGO_TARGET_DIR}/${RUST_BUILD_TARGET})


set(CANVAS_ANDROID_SOURCE_FILE ${PROJECT_NATIVE_ROOT_DIR}/canvas-cxx/src/lib.rs)
set(CANVAS_ANDROID_BRIDGE_CPP ${CARGO_TARGET_DIR}/cxxbridge/canvas-cxx/src/lib.rs.cc)

#set(COMMON_CMAKE_ARGUMENTS "-std=c++17 -pthread -Werror -Wno-unused-result -mstackrealign -fexceptions -fno-builtin-stpcpy -D_LIBCPP_ABI_UNSTABLE -D_LIBCPP_ABI_VERSION=Cr -DV8_31BIT_SMIS_ON_64BIT_ARCH")

set(COMMON_CMAKE_ARGUMENTS "-std=c++17 -pthread -Werror -Wno-unused-result -mstackrealign -fexceptions -fno-builtin-stpcpy -DV8_31BIT_SMIS_ON_64BIT_ARCH")


if (CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(RUST_BUILD_TYPE)
  set(CANVAS_ANDROID_LIB ${CURRENT_TARGET_DIR}/debug)
else ()
  set(RUST_BUILD_TYPE --release)
  set(CANVAS_ANDROID_LIB ${CURRENT_TARGET_DIR}/release)
  set(COMMON_CMAKE_ARGUMENTS "${COMMON_CMAKE_ARGUMENTS} -O3")
endif ()


set(CANVAS_ANDROID_LIB ${CANVAS_ANDROID_LIB}/${CMAKE_STATIC_LIBRARY_PREFIX}canvasandroid${CMAKE_SHARED_LIBRARY_SUFFIX})

if ("${ANDROID_ABI}" MATCHES "arm64-v8a$" OR "${ANDROID_ABI}" MATCHES "x86_64$")
  # Enable pointer compression on 64 bit platforms
  set(COMMON_CMAKE_ARGUMENTS "${COMMON_CMAKE_ARGUMENTS} -DV8_COMPRESS_POINTERS")
endif ()


include_directories(
        canvasnativev8

        PUBLIC

        ${CARGO_TARGET_DIR}/cxxbridge

        ${PROJECT_SOURCE_DIR}/src/main/cpp/include/libc++
        ${PROJECT_SOURCE_DIR}/src/main/cpp/include
        ${PROJECT_SOURCE_DIR}/src/main/cpp
        ${PROJECT_SOURCE_DIR}/src/main/jsi
        ${PROJECT_SOURCE_DIR}/src/main/v8runtime
)

set(CMAKE_CXX_FLAGS ${COMMON_CMAKE_ARGUMENTS})

# Add a custom command that builds the rust crate and generates C++ bridge code
add_custom_command(
        OUTPUT ${CANVAS_ANDROID_BRIDGE_CPP} ${CANVAS_ANDROID_LIB}
       # COMMAND cargo +nightly build -Z build-std='std,panic_abort' -Z build-std-features='panic_immediate_abort' --manifest-path ${CARGO_MANIFEST} --target ${RUST_BUILD_TARGET} ${RUST_BUILD_TYPE} -p canvas-android
        COMMAND cargo build --manifest-path ${CARGO_MANIFEST} --target ${RUST_BUILD_TARGET} ${RUST_BUILD_TYPE} -p canvas-android
        DEPENDS ${CANVAS_ANDROID_SOURCE_FILE}
        USES_TERMINAL
)


add_library(
        canvasnativev8

        SHARED

        src/main/cpp/rust/cxx.cc


        src/main/cpp/jsi/jsi.cpp
        src/main/cpp/jsi/jsilib-posix.cpp
        src/main/cpp/jsi/jsilib-windows.cpp


        src/main/cpp/v8runtime/HostProxy.cpp
        src/main/cpp/v8runtime/JSIV8ValueConverter.cpp
        src/main/cpp/v8runtime/V8PointerValue.cpp
        src/main/cpp/v8runtime/V8Runtime.cpp
        src/main/cpp/v8runtime/V8RuntimeFactory.cpp

        src/main/cpp/OnRafCallback.cpp

        src/main/cpp/RafImpl.cpp

        src/main/cpp/canvas2d/CanvasRenderingContext2DImpl.cpp
        src/main/cpp/canvas2d/CanvasGradient.cpp
        src/main/cpp/canvas2d/CanvasPattern.cpp

        src/main/cpp/canvas2d/MatrixImpl.cpp

        src/main/cpp/canvas2d/ImageDataImpl.cpp

        src/main/cpp/canvas2d/Path2D.cpp

        src/main/cpp/canvas2d/TextMetricsImpl.cpp

        src/main/cpp/ImageAssetImpl.cpp
        src/main/cpp/ImageBitmapImpl.cpp
        src/main/cpp/TextDecoderImpl.cpp
        src/main/cpp/TextEncoderImpl.cpp


        src/main/cpp/webgl/WebGLActiveInfoImpl.cpp
        src/main/cpp/webgl/WebGLShaderPrecisionFormatImpl.cpp


        src/main/cpp/webgl/extensions/ANGLE_instanced_arraysImpl.cpp
        src/main/cpp/webgl/extensions/EXT_blend_minmaxImpl.cpp
        src/main/cpp/webgl/extensions/EXT_color_buffer_half_floatImpl.cpp
        src/main/cpp/webgl/extensions/EXT_disjoint_timer_queryImpl.cpp
        src/main/cpp/webgl/extensions/EXT_sRGBImpl.cpp
        src/main/cpp/webgl/extensions/EXT_texture_filter_anisotropicImpl.cpp
        src/main/cpp/webgl/extensions/OES_element_index_uintImpl.cpp
        src/main/cpp/webgl/extensions/OES_standard_derivativesImpl.cpp
        src/main/cpp/webgl/extensions/OES_texture_half_floatImpl.cpp
        src/main/cpp/webgl/extensions/OES_vertex_array_objectImpl.cpp
        src/main/cpp/webgl/extensions/WEBGL_color_buffer_floatImpl.cpp
        src/main/cpp/webgl/extensions/WEBGL_compressed_texture_atcImpl.cpp
        src/main/cpp/webgl/extensions/WEBGL_compressed_texture_etc1Impl.cpp
        src/main/cpp/webgl/extensions/WEBGL_compressed_texture_etcImpl.cpp
        src/main/cpp/webgl/extensions/WEBGL_compressed_texture_pvrtcImpl.cpp
        src/main/cpp/webgl/extensions/WEBGL_compressed_texture_s3tc_srgbImpl.cpp
        src/main/cpp/webgl/extensions/WEBGL_compressed_texture_s3tcImpl.cpp
        src/main/cpp/webgl/extensions/WEBGL_depth_textureImpl.cpp
        src/main/cpp/webgl/extensions/WEBGL_draw_buffersImpl.cpp
        src/main/cpp/webgl/extensions/WEBGL_lose_contextImpl.cpp


        src/main/cpp/webgl/WebGLRenderingContextBase.cpp
        src/main/cpp/webgl/WebGLRenderingContext.cpp

        src/main/cpp/webgl2/WebGL2RenderingContext.cpp

        src/main/cpp/CanvasJSIModule.cpp

        src/main/cpp/JSIRuntime.cpp

        ${CANVAS_ANDROID_BRIDGE_CPP}
)


message("CARGO_TARGET_DIR " ${CARGO_TARGET_DIR})


set_target_properties(
        canvasnativev8 PROPERTIES
        CXX_STANDARD 17
        CXX_EXTENSIONS OFF
        POSITION_INDEPENDENT_CODE ON
        LINK_FLAGS -Wl,--allow-multiple-definition
)


MESSAGE("# General cmake Info")
MESSAGE("# PROJECT_SOURCE_DIR: " ${PROJECT_SOURCE_DIR})
MESSAGE("# CMAKE_VERSION: " ${CMAKE_VERSION})
MESSAGE("# CMAKE_C_COMPILER_ID: " ${CMAKE_C_COMPILER_ID})
MESSAGE("# CMAKE_CXX_COMPILER_ID: " ${CMAKE_CXX_COMPILER_ID})
MESSAGE("# CMAKE_C_FLAGS: " ${CMAKE_C_FLAGS})
MESSAGE("# CMAKE_CXX_FLAGS: " ${CMAKE_CXX_FLAGS})

MESSAGE("# CARGO_MANIFEST: " ${CARGO_MANIFEST})
MESSAGE("# CARGO_TARGET_DIR: " ${CARGO_TARGET_DIR})
MESSAGE("# CURRENT_TARGET_DIR: " ${CURRENT_TARGET_DIR})
MESSAGE("# CANVAS_ANDROID_SOURCE_FILE: " ${CANVAS_ANDROID_SOURCE_FILE})
MESSAGE("# CANVAS_ANDROID_BRIDGE_CPP: " ${CANVAS_ANDROID_BRIDGE_CPP})
MESSAGE("# ANDROID_NDK_ROOT: " ${ANDROID_NDK_ROOT})

find_library(system-log log)
find_library(system-z z)
find_library(egl EGL)
find_library(gl2 GLESv2)
find_library(gl3 GLESv3)

add_custom_command(TARGET canvasnativev8 POST_BUILD COMMAND ${CMAKE_STRIP} --strip-unneeded $<TARGET_FILE:canvasnativev8>)

if ("${ANDROID_ABI}" MATCHES "armeabi-v7a" OR "${ANDROID_ABI}" MATCHES "x86" AND NOT "${ANDROID_ABI}" MATCHES "x86_64")
  # On API Level 19 and lower we need to link with android_support
  # because it contains some implementation of functions such as "strtoll" and "strtoul"
  target_link_libraries(canvasnativev8
          ${system-log}
          ${system-z}
          ${egl}
          ${gl2}
          ${gl3}
          ${PROJECT_SOURCE_DIR}/src/main/libs/${ANDROID_ABI}/libNativeScript.so
          ${ANDROID_NDK_ROOT}/sources/cxx-stl/llvm-libc++/libs/${ANDROID_ABI}/libandroid_support.a
          ${CANVAS_ANDROID_LIB}
          android
          )

else ()
  target_link_libraries(canvasnativev8
          ${system-log}
          ${system-z}
          ${egl}
          ${gl2}
          ${gl3}
          ${PROJECT_SOURCE_DIR}/src/main/libs/${ANDROID_ABI}/libNativeScript.so
          ${CANVAS_ANDROID_LIB}
          android
          )
endif ()


add_custom_command(TARGET canvasnativev8
        POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
        ${CANVAS_ANDROID_LIB}
        ${CMAKE_LIBRARY_OUTPUT_DIRECTORY}/libcanvasandroid.so
        )

