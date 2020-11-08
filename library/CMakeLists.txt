project(library) 

set(library_SRC
    src/library.cpp)

corrosion_import_crate(MANIFEST_PATH Cargo.toml)

add_library(mcrestool-library SHARED ${library_SRC})

target_link_libraries(mcrestool-library pthread dl mcrestool-library-rs)
