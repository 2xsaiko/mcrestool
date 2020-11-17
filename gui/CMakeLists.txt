project(gui)

set(mcrestool_INCLUDES
        model
        project
        ui
        workspace)

set(mcrestool_HEADERS
        model/languagetablemodel.h
        project/languagetablecontainer.h
        ui/geneditorwindow.h
        ui/itembutton.h
        ui/languagetablewindow.h
        ui/mainwindow.h
        ui/modeleditwindow.h
        ui/recipeeditextensionwidget.h
        ui/recipeeditwindow.h
        ui/shapedcraftingwidget.h
        ui/smeltingwidget.h
        workspace/fstreemodel.h
        identifier.h
        mcrtutil.h
        result.h
        table.h)

set(mcrestool_SRC
        model/languagetablemodel.cpp
        project/languagetablecontainer.cpp
        ui/geneditorwindow.cpp
        ui/itembutton.cpp
        ui/languagetablewindow.cpp
        ui/mainwindow.cpp
        ui/modeleditwindow.cpp
        ui/recipeeditextensionwidget.cpp
        ui/recipeeditwindow.cpp
        ui/shapedcraftingwidget.cpp
        ui/smeltingwidget.cpp
        workspace/fstreemodel.cpp
        identifier.cpp
        main.cpp)

add_executable(mcrestool ${mcrestool_SRC})

target_include_directories(mcrestool PRIVATE ${mcrestool_INCLUDES})

set_property(TARGET mcrestool PROPERTY CXX_STANDARD 20)

target_link_libraries(mcrestool PUBLIC
        Qt5::Core
        Qt5::Widgets
        mcrtlib)
        

install(TARGETS mcrestool DESTINATION bin)
