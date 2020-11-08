project(gui)

set(mcrestool_INCLUDES
        languagetable
        model
        project
        ui
        workspace)

set(mcrestool_HEADERS
        languagetable/languagetable.h
        model/languagetablemodel.h
        project/languagetablecontainer.h
        ui/geneditorwindow.h
        ui/itembutton.h
        ui/languagetablewindow.h
        ui/mainwindow.h
        ui/recipeeditextensionwidget.h
        ui/recipeeditwindow.h
        ui/shapedcraftingwidget.h
        ui/smeltingwidget.h
        workspace/direntry.h
        workspace/filetype.h
        workspace/fsref.h
        workspace/fstree.h
        workspace/fstreemodel.h
        workspace/path.h
        workspace/workspace.h
        identifier.h
        mcrtutil.h
        result.h
        table.h)

set(mcrestool_SRC
        languagetable/languagetable.cpp
        model/languagetablemodel.cpp
        project/languagetablecontainer.cpp
        ui/geneditorwindow.cpp
        ui/itembutton.cpp
        ui/languagetablewindow.cpp
        ui/mainwindow.cpp
        ui/recipeeditextensionwidget.cpp
        ui/recipeeditwindow.cpp
        ui/shapedcraftingwidget.cpp
        ui/smeltingwidget.cpp
        workspace/fsref.cpp
        workspace/fstree.cpp
        workspace/fstreemodel.cpp
        workspace/path.cpp
        workspace/workspace.cpp
        identifier.cpp
        main.cpp)

add_executable(mcrestool ${mcrestool_SRC})

target_include_directories(mcrestool PRIVATE ${mcrestool_INCLUDES})

set_property(TARGET mcrestool PROPERTY CXX_STANDARD 20)

target_link_libraries(mcrestool PUBLIC
        Qt5::Core
        Qt5::Widgets
        KF5::Archive
        QuaZip::QuaZip
        mcrestool-library)
        

install(TARGETS mcrestool DESTINATION bin)
