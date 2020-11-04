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

find_package(QuaZip-Qt5 REQUIRED)

set(mcrestool_INCLUDES
        src/
        src/languagetable
        src/model
        src/project
        src/ui
        src/workspace)

set(mcrestool_HEADERS
        src/languagetable/languagetable.h
        src/model/languagetablemodel.h
        src/project/languagetablecontainer.h
        src/ui/geneditorwindow.h
        src/ui/itembutton.h
        src/ui/languagetablewindow.h
        src/ui/mainwindow.h
        src/ui/recipeeditextensionwidget.h
        src/ui/recipeeditwindow.h
        src/ui/shapedcraftingwidget.h
        src/ui/smeltingwidget.h
        src/workspace/direntry.h
        src/workspace/filetype.h
        src/workspace/fsref.h
        src/workspace/fstree.h
        src/workspace/fstreemodel.h
        src/workspace/path.h
        src/workspace/workspace.h
        src/identifier.h
        src/mcrtutil.h
        src/result.h
        src/table.h)

set(mcrestool_SRC
        src/languagetable/languagetable.cpp
        src/model/languagetablemodel.cpp
        src/project/languagetablecontainer.cpp
        src/ui/geneditorwindow.cpp
        src/ui/itembutton.cpp
        src/ui/languagetablewindow.cpp
        src/ui/mainwindow.cpp
        src/ui/recipeeditextensionwidget.cpp
        src/ui/recipeeditwindow.cpp
        src/ui/shapedcraftingwidget.cpp
        src/ui/smeltingwidget.cpp
        src/workspace/fsref.cpp
        src/workspace/fstree.cpp
        src/workspace/fstreemodel.cpp
        src/workspace/path.cpp
        src/workspace/workspace.cpp
        src/identifier.cpp
        src/main.cpp)

add_executable(mcrestool ${mcrestool_SRC})

target_include_directories(mcrestool PRIVATE ${mcrestool_INCLUDES})

set_property(TARGET mcrestool PROPERTY CXX_STANDARD 20)

target_link_libraries(mcrestool
        Qt5::Core
        Qt5::Widgets
        KF5::Archive
        QuaZip::QuaZip)

install(TARGETS mcrestool DESTINATION bin)
