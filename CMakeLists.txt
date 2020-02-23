cmake_minimum_required(VERSION 3.0)

project(mcrestool)

set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")

enable_language(Rust)
include(CMakeCargo)

set(LIB_BUILD_DIR ${CMAKE_BINARY_DIR}/lib)
include_directories(${LIB_BUILD_DIR})

add_subdirectory(lib)
add_subdirectory(ui)

