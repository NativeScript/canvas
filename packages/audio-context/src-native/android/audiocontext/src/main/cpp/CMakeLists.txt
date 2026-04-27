cmake_minimum_required(VERSION 3.6)
project(native-audio)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

option(OBOE_DIR "Path to Oboe source directory (optional)" "")

# Prefer a bundled submodule under the native module if present.
if(NOT OBOE_DIR)
    set(POSSIBLE_OBOE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/third_party/oboe")
    if(EXISTS ${POSSIBLE_OBOE_DIR})
        set(OBOE_DIR ${POSSIBLE_OBOE_DIR})
        message(STATUS "Found bundled Oboe at: ${OBOE_DIR}")
    endif()
endif()

if (OBOE_DIR)
    message(STATUS "Using Oboe from: ${OBOE_DIR}")
    add_subdirectory(${OBOE_DIR} ${CMAKE_CURRENT_BINARY_DIR}/oboe_build)
    set(HAS_OBOE 1)
endif()

add_library(native-audio SHARED
    audio_context.cpp
    audio_render.cpp
    audio_offline.cpp
    audio_jni.cpp)

# Include small bundled kissfft implementation
target_sources(native-audio PRIVATE third_party/kissfft/kiss_fft.c)
target_include_directories(native-audio PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/third_party/kissfft)

find_library(log-lib log)

target_include_directories(native-audio PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

if (HAS_OBOE)
    target_link_libraries(native-audio PRIVATE oboe ${log-lib})
    target_compile_definitions(native-audio PRIVATE HAS_OBOE=1)
else()
    find_library(AAUDIO_LIB aaudio)
    if (AAUDIO_LIB)
        target_link_libraries(native-audio PRIVATE ${AAUDIO_LIB} ${log-lib})
    else()
        target_link_libraries(native-audio PRIVATE ${log-lib})
    endif()
    target_compile_definitions(native-audio PRIVATE HAS_OBOE=0)
endif()
