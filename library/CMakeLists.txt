project(library) 

set(library_INCLUDES
        src/)

set(library_SRC
        src/library.cpp)

corrosion_import_crate(MANIFEST_PATH Cargo.toml)
set(BRIDGE_OUT "${CMAKE_CURRENT_BINARY_DIR}/lib.rs.cpp")
add_custom_command(
        OUTPUT ${BRIDGE_OUT}
        COMMAND ${CXXBRIDGE} src/lib.rs --header > ${CMAKE_CURRENT_BINARY_DIR}/lib.rs.h
        COMMAND ${CXXBRIDGE} src/lib.rs > ${BRIDGE_OUT}
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
        DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/src/lib.rs)

add_library(mcrestool-library SHARED ${library_SRC} ${BRIDGE_OUT})
add_definitions(-DMCRTLIB_BUILD)

target_include_directories(mcrestool-library
        PUBLIC include/
        PRIVATE ${library_INCLUDES})

target_link_libraries(mcrestool-library pthread dl mcrestool-library-rs)
