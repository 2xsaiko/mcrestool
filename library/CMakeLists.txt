project(mcrtlib)

set(mcrtlib_INCLUDES
        src/)

set(mcrtlib_SRC
        src/library.cpp)

corrosion_import_crate(MANIFEST_PATH Cargo.toml)
set(BRIDGE_IN "${CMAKE_CURRENT_SOURCE_DIR}/src/ffi.rs")
set(BRIDGE_OUT "${CMAKE_CURRENT_BINARY_DIR}/lib.rs.cpp")
set(BRIDGE_ARGS -istdexcept)
# stdexcept here added because otherwise it's trying to use a type that isn't
# included
add_custom_command(
        OUTPUT ${BRIDGE_OUT}
        COMMAND ${CXXBRIDGE} ${BRIDGE_IN} ${BRIDGE_ARGS} --header > ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.h
        COMMAND ${CXXBRIDGE} ${BRIDGE_IN} ${BRIDGE_ARGS} > ${BRIDGE_OUT}
        DEPENDS ${BRIDGE_IN})

add_library(mcrtlib STATIC ${mcrtlib_SRC} ${BRIDGE_OUT})
add_definitions(-DMCRTLIB_BUILD)

target_include_directories(mcrtlib
        PUBLIC include/
        PRIVATE ${mcrtlib_INCLUDES})

target_link_libraries(mcrtlib PUBLIC
        Qt5::Core
        pthread
        dl
        mcrtlib-rs)
