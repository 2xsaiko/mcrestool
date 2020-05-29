#ifndef MCRESTOOL_GENEDITORWINDOW_H
#define MCRESTOOL_GENEDITORWINDOW_H

class GenEditorWindow {

public:
    virtual ~GenEditorWindow() = default;

    virtual void save() = 0;

    virtual void reload() = 0;

};

#endif //MCRESTOOL_GENEDITORWINDOW_H
