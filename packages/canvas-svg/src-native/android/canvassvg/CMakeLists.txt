cmake_minimum_required(VERSION 3.4.1)
set(CMAKE_VERBOSE_MAKEFILE ON)

project(canvassvgnativev8)

set(CMAKE_CXX_STANDARD 20)

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
get_filename_component(PROJECT_NATIVE_ROOT_DIR "${PROJECT_ROOT_DIR}/../../../../.." DIRECTORY)
set(PROJECT_NATIVE_ROOT_DIR "${PROJECT_NATIVE_ROOT_DIR}")

set(CARGO_TARGET_DIR ${PROJECT_NATIVE_ROOT_DIR}/target)
set(CURRENT_TARGET_DIR ${CARGO_TARGET_DIR}/${RUST_BUILD_TARGET})

set(COMMON_CMAKE_ARGUMENTS "-std=c++20 -pthread -Werror -Wno-unused-result -mstackrealign -fexceptions -fno-builtin-stpcpy -DV8_31BIT_SMIS_ON_64BIT_ARCH -Wno-vla-extension -Wno-deprecated")

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(CANVAS_SVG_ANDROID_LIB ${CURRENT_TARGET_DIR}/debug)
else ()
  set(CANVAS_SVG_ANDROID_LIB ${CURRENT_TARGET_DIR}/release)
  set(COMMON_CMAKE_ARGUMENTS "${COMMON_CMAKE_ARGUMENTS} -O3")
endif ()

set(CANVAS_SVG_ANDROID_LIB ${CANVAS_SVG_ANDROID_LIB}/${CMAKE_STATIC_LIBRARY_PREFIX}canvassvg${CMAKE_SHARED_LIBRARY_SUFFIX})

if ("${ANDROID_ABI}" MATCHES "arm64-v8a$" OR "${ANDROID_ABI}" MATCHES "x86_64$")
  set(COMMON_CMAKE_ARGUMENTS "${COMMON_CMAKE_ARGUMENTS} -DV8_COMPRESS_POINTERS")
endif ()

set(CMAKE_CXX_FLAGS ${COMMON_CMAKE_ARGUMENTS})

set(CPP_DIR ../../../platforms/ios/src/cpp)

add_definitions(-DTARGET_OS_ANDROID)

# Borrows canvas's vendored V8 headers/libNativeScript.so (build-time only, not a
# package dependency).
set(CANVAS_ANDROID_MODULE_DIR ${PROJECT_NATIVE_ROOT_DIR}/packages/canvas/src-native/canvas-android/canvas)

include_directories(
        canvassvgnativev8
        PUBLIC
        ${PROJECT_SOURCE_DIR}/src/main/cpp
        ${PROJECT_SOURCE_DIR}/src/main/cpp/include
        ${CANVAS_ANDROID_MODULE_DIR}/src/main/cpp
        ${CANVAS_ANDROID_MODULE_DIR}/src/main/cpp/include/libc++
        ${CANVAS_ANDROID_MODULE_DIR}/src/main/cpp/include
        ${CPP_DIR}
)

set(CANVAS_SVG_C ${PROJECT_NATIVE_ROOT_DIR}/crates/canvas-svg-c)
set(CANVAS_SVG_INCLUDE ${PROJECT_SOURCE_DIR}/src/main/cpp/include)

add_custom_command(
        OUTPUT ${CANVAS_SVG_INCLUDE}/canvas_native_svg.h
        COMMAND cbindgen --config ${CANVAS_SVG_C}/cbindgen.toml ${CANVAS_SVG_C}/src/lib.rs -l c > ${CANVAS_SVG_INCLUDE}/canvas_native_svg.h
        USES_TERMINAL
)

add_library(
        canvassvgnativev8

        SHARED

        ${CPP_DIR}/SVGDocumentImpl.cpp
        ${CPP_DIR}/SVGNodeImpl.cpp
        ${CPP_DIR}/SVGJSIModule.cpp
        ${CPP_DIR}/SVGJSIRuntime.cpp

        ${CANVAS_SVG_INCLUDE}/canvas_native_svg.h
)

set_target_properties(
        canvassvgnativev8 PROPERTIES
        CXX_STANDARD 20
        CXX_EXTENSIONS OFF
        POSITION_INDEPENDENT_CODE ON
        LINK_FLAGS -Wl,--allow-multiple-definition
)

find_library(system-log log)

target_link_libraries(canvassvgnativev8
        ${system-log}
        ${CANVAS_ANDROID_MODULE_DIR}/src/main/jniLibs/${ANDROID_ABI}/libNativeScript.so
        ${CANVAS_SVG_ANDROID_LIB}
        android
)

target_link_options(canvassvgnativev8 PRIVATE "-Wl,-z,max-page-size=16384")

add_custom_command(TARGET canvassvgnativev8 POST_BUILD COMMAND ${CMAKE_STRIP} --strip-unneeded $<TARGET_FILE:canvassvgnativev8>)
