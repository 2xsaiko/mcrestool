project(library) 

set(library_INCLUDES
        src/)

set(library_SRC
        src/library.cpp)

corrosion_import_crate(MANIFEST_PATH Cargo.toml)
set(BRIDGE_IN "${CMAKE_CURRENT_SOURCE_DIR}/src/ffi.rs")
set(BRIDGE_OUT "${CMAKE_CURRENT_BINARY_DIR}/lib.rs.cpp")
# stdexcept here added because otherwise it's trying to use a type that isn't
# included
add_custom_command(
        OUTPUT ${BRIDGE_OUT}
        COMMAND ${CXXBRIDGE} ${BRIDGE_IN} -istdexcept --header > ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.h
        COMMAND ${CXXBRIDGE} ${BRIDGE_IN} -istdexcept > ${BRIDGE_OUT}
        DEPENDS ${BRIDGE_IN})

add_library(mcrestool-library STATIC ${library_SRC} ${BRIDGE_OUT})
add_definitions(-DMCRTLIB_BUILD)

target_include_directories(mcrestool-library
        PUBLIC include/
        PRIVATE ${library_INCLUDES})

target_link_libraries(mcrestool-library PUBLIC
        Qt5::Core
        pthread
        dl
        mcrestool-library-rs)
