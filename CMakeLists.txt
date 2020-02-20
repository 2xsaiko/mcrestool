cmake_minimum_required(VERSION 3.0)
project(mcrestool)

set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_AUTOUIC ON)

find_package(ECM REQUIRED NO_MODULE)
set(CMAKE_MODULE_PATH ${ECM_MODULE_PATH})

include(KDEInstallDirs NO_POLICY_SCOPE)
include(KDECompilerSettings NO_POLICY_SCOPE)
include(KDECMakeSettings NO_POLICY_SCOPE)

find_package(Qt5 CONFIG REQUIRED
        Core
        Widgets)

find_package(KF5 REQUIRED
        Archive)

set(mcrestool_SRC
        src/main.cpp
        src/model/langtabl.cpp src/model/langtabl.h
        src/table.h
        src/identifr.cpp src/identifr.h
        src/model/restree.cpp src/model/restree.h
        src/ui/mainwin.cpp src/ui/mainwin.h
        src/ui/langtblw.cpp src/ui/langtblw.h
        src/ui/itembtn.cpp src/ui/itembtn.h
        src/ui/recpedtw.cpp src/ui/recpedtw.h
        src/ui/recpshcr.cpp src/ui/recpshcr.h
        src/ui/rcpedext.cpp src/ui/rcpedext.h
        src/util.h
        src/fs/langfile.cpp src/fs/langfile.h
        src/result.h
        src/except.h
        src/ui/recpsmel.cpp src/ui/recpsmel.h
        src/model/restritm.cpp src/model/restritm.h
        src/project/projsrc.cpp src/project/projsrc.h
        src/fs/datasrc.cpp src/fs/datasrc.h
        src/fs/dirdtsrc.cpp src/fs/dirdtsrc.h
        src/fs/arcdtsrc.cpp src/fs/arcdtsrc.h
        src/project/langtblc.cpp src/project/langtblc.h
        src/project/project.h)

add_executable(mcrestool ${mcrestool_SRC})

set_property(TARGET mcrestool PROPERTY CXX_STANDARD 20)

target_link_libraries(mcrestool
        Qt5::Core
        Qt5::Widgets
        KF5::Archive)

install(TARGETS mcrestool DESTINATION bin)
