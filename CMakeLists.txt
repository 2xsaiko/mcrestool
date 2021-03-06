cmake_minimum_required(VERSION 3.0)
project(mcrestool)

set(CMAKE_CXX_STANDARD_REQUIRED ON)
cmake_policy(SET CMP0071 NEW)

set(CMAKE_AUTOUIC ON)

find_package(ECM REQUIRED NO_MODULE)
set(CMAKE_MODULE_PATH ${ECM_MODULE_PATH})

set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -Og")

include(KDEInstallDirs NO_POLICY_SCOPE)
include(KDECompilerSettings NO_POLICY_SCOPE)
include(KDECMakeSettings NO_POLICY_SCOPE)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${PROJECT_BINARY_DIR})
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${PROJECT_BINARY_DIR})

find_package(Corrosion REQUIRED)
find_program(CXXBRIDGE cxxbridge REQUIRED PATHS $ENV{HOME}/.cargo/bin)
message(STATUS "Using cxxbridge: ${CXXBRIDGE}")

find_package(Qt5 CONFIG REQUIRED
        Core
        Widgets)

add_definitions(-fexceptions)

add_subdirectory(library)
add_subdirectory(gui)
