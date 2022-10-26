if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build --target ${RUST_BUILD_TARGET} --package canvas-android)
    set(TARGET_DIR ${RUST_BUILD_TARGET}/debug)
else ()
    set(CARGO_CMD cargo build --release --target ${RUST_BUILD_TARGET} --package canvas-android)
    set(TARGET_DIR ${RUST_BUILD_TARGET}/release)
endif ()


if(ENABLE_LTO)
    set(RUST_FLAGS "-Clinker-plugin-lto" "-Clinker=clang-14" "-Clink-arg=-fuse-ld=lld-14")
endif()

set(CANVAS_CORE_LIB ${PROJECT_NATIVE_ROOT_DIR}/target/${TARGET_DIR}/libcanvasandroid.a)

set(CANVAS_CXX ${CMAKE_CURRENT_BINARY_DIR}/canvas_core.cpp)
set(CANVAS_CXX_HEADER ${CMAKE_CURRENT_BINARY_DIR}/canvas_core.h)
set(RUST_CXX_HEADER ${PROJECT_BINARY_DIR}/rust/cxx.h)

# Update if CFG.include_prefix changes
set(CXX_BRIDGE_PREFIX "")


include_directories(
        ${PROJECT_NATIVE_ROOT_DIR}/target/${RUST_BUILD_TARGET}/cxxbridge
)

add_custom_command(
        OUTPUT "${PROJECT_NATIVE_ROOT_DIR}/target/${RUST_BUILD_TARGET}/cxxbridge/canvas-android/lib.rs.cc"
        COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} RUSTFLAGS="${RUST_FLAGS}" ${CARGO_CMD}
        WORKING_DIRECTORY ${PROJECT_NATIVE_ROOT_DIR}/canvas-native
)

add_custom_command(
        OUTPUT ${BLOBSTORE_BRIDGE_CPP} ${BLOBSTORE_LIB}
        COMMAND cargo build --manifest-path ${CARGO_MANIFEST}
        DEPENDS ${BLOBSTORE_SOURCE_FILE}
        USES_TERMINAL
)

#add_library(canvas_core STATIC ${PROJECT_NATIVE_ROOT_DIR}/target/${RUST_BUILD_TARGET}/cxxbridge/canvas-android/lib.rs.cc)
#
#target_link_libraries(canvas_core ${CANVAS_CORE_LIB})

#add_test(NAME canvas_core_test
#        COMMAND cargo test
#        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
