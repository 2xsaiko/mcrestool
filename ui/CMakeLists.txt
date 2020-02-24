cmake_minimum_required(VERSION 3.0)
project(mcrestool_ui)

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

set(mcrestool_ui_SRC
        src/main.cpp
        src/model/languagetable.cpp src/model/languagetable.h
        src/table.h
        src/identifier.cpp src/identifier.h
        src/model/resourcetree.cpp src/model/resourcetree.h
        src/ui/mainwindow.cpp src/ui/mainwindow.h
        src/ui/languagetablewindow.cpp src/ui/languagetablewindow.h
        src/ui/itembutton.cpp src/ui/itembutton.h
        src/ui/recipeeditwindow.cpp src/ui/recipeeditwindow.h
        src/ui/shapedcraftingwidget.cpp src/ui/shapedcraftingwidget.h
        src/ui/recipeeditextensionwidget.cpp src/ui/recipeeditextensionwidget.h
        src/util.h
        src/fs/langfile.cpp src/fs/langfile.h
        src/result.h
        src/except.h
        src/ui/smeltingwidget.cpp src/ui/smeltingwidget.h
        src/model/treeitem.cpp src/model/treeitem.h
        src/project/projectsource.cpp src/project/projectsource.h
        src/fs/datasource.cpp src/fs/datasource.h
        src/project/languagetablecontainer.cpp src/project/languagetablecontainer.h
        src/project/project.h)

add_executable(mcrestool_ui ${mcrestool_ui_SRC})

set_property(TARGET mcrestool_ui PROPERTY CXX_STANDARD 20)

target_link_libraries(mcrestool_ui
        mcrestool_logic
        Qt5::Core
        Qt5::Widgets
        KF5::Archive)

install(TARGETS mcrestool_ui DESTINATION bin)
