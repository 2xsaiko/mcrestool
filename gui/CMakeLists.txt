project(gui)

set(mcrestool_INCLUDES
        model
        project
        ui)

set(mcrestool_HEADERS
        model/fstreemodel.h
        model/gobjtreemodel.h
        model/languagetablemodel.h
        model/rustitemmodel.h
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
        identifier.h
        mcrtutil.h)

set(mcrestool_SRC
        model/fstreemodel.cpp
        model/gobjtreemodel.cpp
        model/languagetablemodel.cpp
        model/rustitemmodel.cpp
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
