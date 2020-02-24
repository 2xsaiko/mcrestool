cmake_minimum_required(VERSION 3.0)

project(mcrestool)

set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")

enable_language(Rust)
include(CMakeCargo)

set(LOGIC_BUILD_DIR ${CMAKE_BINARY_DIR}/logic)
include_directories(${LOGIC_BUILD_DIR})

add_subdirectory(logic)
add_subdirectory(ui)

