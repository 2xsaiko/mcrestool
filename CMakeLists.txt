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

find_package(QuaZip5 REQUIRED)

set(mcrestool_HEADERS
        src/model/languagetablemodel.h
        src/table.h
        src/identifier.h
        src/model/resourcetree.h
        src/ui/mainwindow.h
        src/ui/languagetablewindow.h
        src/ui/itembutton.h
        src/ui/recipeeditwindow.h
        src/ui/shapedcraftingwidget.h
        src/ui/recipeeditextensionwidget.h
        src/util.h
        src/result.h
        src/ui/smeltingwidget.h
        src/model/treeitem.h
        src/project/languagetablecontainer.h
        src/ui/geneditorwindow.h
        src/workspace/workspace.h
        src/workspace/fsref.h
        src/languagetable/languagetable.h
        src/workspace/direntry.h)

set(mcrestool_SRC
        src/main.cpp
        src/model/languagetablemodel.cpp
        src/identifier.cpp
        src/model/resourcetree.cpp
        src/ui/mainwindow.cpp
        src/ui/languagetablewindow.cpp
        src/ui/itembutton.cpp
        src/ui/recipeeditwindow.cpp
        src/ui/shapedcraftingwidget.cpp
        src/ui/recipeeditextensionwidget.cpp
        src/ui/smeltingwidget.cpp
        src/model/treeitem.cpp
        src/project/languagetablecontainer.cpp
        src/ui/geneditorwindow.cpp
        src/workspace/workspace.cpp
        src/workspace/fsref.cpp
        src/languagetable/languagetable.cpp)

add_executable(mcrestool ${mcrestool_SRC})

set_property(TARGET mcrestool PROPERTY CXX_STANDARD 20)

target_link_libraries(mcrestool
        Qt5::Core
        Qt5::Widgets
        KF5::Archive
        ${QUAZIP_LIBRARIES})

install(TARGETS mcrestool DESTINATION bin)