cmake_minimum_required(VERSION 3.0)
project(mcrestool)

set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)

find_package(Qt5Core CONFIG REQUIRED)
find_package(Qt5Widgets CONFIG REQUIRED)

set(mcrestool_SRC
        src/main.cpp
        src/model/langtbl.cpp src/model/langtbl.h
        src/table.h
        src/identifr.cpp src/identifr.h
        src/model/restree.cpp src/model/restree.h
        src/ui/mainwin.cpp src/ui/mainwin.h
        src/ui/langtblw.cpp src/ui/langtblw.h
        src/ui/itembtn.cpp src/ui/itembtn.h src/ui/recpedtw.cpp src/ui/recpedtw.h src/ui/recpshcr.cpp src/ui/recpshcr.h src/ui/rcpedext.cpp src/ui/rcpedext.h src/util.h src/fs/langfile.cpp src/fs/langfile.h src/result.h src/except.h)

add_executable(mcrestool ${mcrestool_SRC})

set_property(TARGET mcrestool PROPERTY CXX_STANDARD 20)

target_link_libraries(mcrestool Qt5::Core)
target_link_libraries(mcrestool Qt5::Widgets)
#target_link_libraries(mcrestool quazip)

install(TARGETS mcrestool DESTINATION bin)
