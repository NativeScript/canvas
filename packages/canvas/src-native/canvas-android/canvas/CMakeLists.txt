cmake_minimum_required(VERSION 3.22.0)
set(CMAKE_VERBOSE_MAKEFILE ON)

project(canvasnativev8)

set(CMAKE_CXX_STANDARD 17)

#option(ENABLE_LTO "Enable cross language linking time optimization" ON)
#if (ENABLE_LTO)
#    include(CheckIPOSupported)
#    check_ipo_supported(RESULT supported OUTPUT error)
#    if (supported)
#        message(STATUS "IPO / LTO enabled")
#        set(CMAKE_INTERPROCEDURAL_OPTIMIZATION TRUE)
#        add_link_options(-fuse-ld=lld-14)
#    else ()
#        message(STATUS "IPO / LTO not supported: <${error}>")
#    endif ()
#endif ()

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

set(COMMON_CMAKE_ARGUMENTS "-std=c++17 -pthread -Werror -Wno-unused-result -mstackrealign -fexceptions -fno-builtin-stpcpy -DV8_31BIT_SMIS_ON_64BIT_ARCH")

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(RUST_BUILD_TYPE)
  set(CANVAS_ANDROID_LIB ${CURRENT_TARGET_DIR}/debug)
else ()
  set(RUST_BUILD_TYPE --release)
  set(CANVAS_ANDROID_LIB ${CURRENT_TARGET_DIR}/release)
  set(COMMON_CMAKE_ARGUMENTS "${COMMON_CMAKE_ARGUMENTS} -O3")
endif ()


set(CANVAS_ANDROID_LIB ${CANVAS_ANDROID_LIB}/${CMAKE_STATIC_LIBRARY_PREFIX}canvasnative${CMAKE_SHARED_LIBRARY_SUFFIX})

message(CANVAS_ANDROID_LIB ${CANVAS_ANDROID_LIB})

if ("${ANDROID_ABI}" MATCHES "arm64-v8a$" OR "${ANDROID_ABI}" MATCHES "x86_64$")
  # Enable pointer compression on 64 bit platforms
  set(COMMON_CMAKE_ARGUMENTS "${COMMON_CMAKE_ARGUMENTS} -DV8_COMPRESS_POINTERS")
endif ()


if (${ANDROID_PLATFORM_LEVEL} LESS 18)
  add_definitions("-DDYNAMIC_ES3")
  set(GL3STUB_SRC src/main/cpp/gl3stub.c)
  set(OPENGL_LIB GLESv2)
else ()
  set(OPENGL_LIB GLESv2 GLESv3)
endif ()


set(CPP_DIR ../../../platforms/ios/src/cpp)

add_definitions(-DTARGET_OS_ANDROID)

include_directories(
        canvasnativev8
        PUBLIC
        ${PROJECT_SOURCE_DIR}/src/main/cpp
        ${PROJECT_SOURCE_DIR}/src/main/cpp/include/libc++
        ${PROJECT_SOURCE_DIR}/src/main/cpp/include
        ../../../platforms/ios/src/cpp
        ../../../platforms/ios/src/cpp/canvas2d
        ../../../platforms/ios/src/cpp/webgl
        ../../../platforms/ios/src/cpp/webgl/extensions
        ../../../platforms/ios/src/cpp/webgl2
)

set(CMAKE_CXX_FLAGS ${COMMON_CMAKE_ARGUMENTS})

set(CANVAS_C ${PROJECT_NATIVE_ROOT_DIR}/canvas-c)
set(CANVAS_ANDROID ${PROJECT_NATIVE_ROOT_DIR}/canvas-android)
set(CANVAS_INCLUDE ${PROJECT_SOURCE_DIR}/src/main/cpp/include)

add_custom_command(
        OUTPUT ${CANVAS_INCLUDE}/canvas_android.h
        COMMAND cbindgen --config ${CANVAS_C}/cbindgen.toml ${CANVAS_C}/src/lib.rs -l c > ${CANVAS_INCLUDE}/canvas_native.h
        USES_TERMINAL
)


add_library(
        canvasnativev8

        SHARED

        ${GL3STUB_SRC}

        ${CPP_DIR}/Caches.cpp

        ${CPP_DIR}/OneByteStringResource.cpp

        ${CPP_DIR}/OnRafCallback.cpp

        ${CPP_DIR}/RafImpl.cpp

        ${CPP_DIR}/canvas2d/CanvasRenderingContext2DImpl.cpp
        ${CPP_DIR}/canvas2d/CanvasGradient.cpp
        ${CPP_DIR}/canvas2d/CanvasPattern.cpp

        ${CPP_DIR}/canvas2d/MatrixImpl.cpp

        ${CPP_DIR}/canvas2d/ImageDataImpl.cpp

        ${CPP_DIR}/canvas2d/Path2D.cpp

        ${CPP_DIR}/canvas2d/TextMetricsImpl.cpp

        ${CPP_DIR}/ImageAssetImpl.cpp
        ${CPP_DIR}/ImageBitmapImpl.cpp
        ${CPP_DIR}/TextDecoderImpl.cpp
        ${CPP_DIR}/TextEncoderImpl.cpp
        ${CPP_DIR}/URLImpl.cpp


        ${CPP_DIR}/webgl/WebGLActiveInfoImpl.cpp
        ${CPP_DIR}/webgl/WebGLShaderPrecisionFormatImpl.cpp


        ${CPP_DIR}/webgl/extensions/ANGLE_instanced_arraysImpl.cpp
        ${CPP_DIR}/webgl/extensions/EXT_blend_minmaxImpl.cpp
        ${CPP_DIR}/webgl/extensions/EXT_color_buffer_half_floatImpl.cpp
        ${CPP_DIR}/webgl/extensions/EXT_disjoint_timer_queryImpl.cpp
        ${CPP_DIR}/webgl/extensions/OES_vertex_array_objectImpl.cpp
        ${CPP_DIR}/webgl/extensions/WEBGL_draw_buffersImpl.cpp
        ${CPP_DIR}/webgl/extensions/WEBGL_lose_contextImpl.cpp


        ${CPP_DIR}/webgl/WebGLRenderingContextBase.cpp
        ${CPP_DIR}/webgl/WebGLRenderingContext.cpp

        ${CPP_DIR}/webgl2/WebGL2RenderingContext.cpp

        ${CPP_DIR}/CanvasJSIModule.cpp

        ${CPP_DIR}/JSIRuntime.cpp
)


set_target_properties(
        canvasnativev8 PROPERTIES
        CXX_STANDARD 17
        CXX_EXTENSIONS OFF
        POSITION_INDEPENDENT_CODE ON
        LINK_FLAGS -Wl,--allow-multiple-definition
)

find_library(system-log log)
find_library(system-z z)
find_library(egl EGL)

add_custom_command(TARGET canvasnativev8 POST_BUILD COMMAND ${CMAKE_STRIP} --strip-unneeded $<TARGET_FILE:canvasnativev8>)

if ("${ANDROID_ABI}" MATCHES "armeabi-v7a" OR "${ANDROID_ABI}" MATCHES "x86" AND NOT "${ANDROID_ABI}" MATCHES "x86_64")
  # On API Level 19 and lower we need to link with android_support
  # because it contains some implementation of functions such as "strtoll" and "strtoul"
  target_link_libraries(canvasnativev8
          ${system-log}
          ${system-z}
          ${egl}
          ${OPENGL_LIB}
          ${PROJECT_SOURCE_DIR}/src/main/java/jniLibs/${ANDROID_ABI}/libNativeScript.so
          #${ANDROID_NDK_ROOT}/sources/cxx-stl/llvm-libc++/libs/${ANDROID_ABI}/libandroid_support.a
          ${CANVAS_ANDROID_LIB}
          android
          )

else ()
  target_link_libraries(canvasnativev8
          ${system-log}
          ${system-z}
          ${egl}
          ${OPENGL_LIB}
          ${graphics}
          ${PROJECT_SOURCE_DIR}/src/main/java/jniLibs/${ANDROID_ABI}/libNativeScript.so
          ${CANVAS_ANDROID_LIB}
          android
          )
endif ()


add_custom_command(TARGET canvasnativev8
        PRE_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
        ${CANVAS_ANDROID_LIB}
        ${CMAKE_LIBRARY_OUTPUT_DIRECTORY}/libcanvasnative.so
        )

