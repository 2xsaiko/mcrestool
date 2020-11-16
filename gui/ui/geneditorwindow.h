#ifndef MCRESTOOL_GENEDITORWINDOW_H
#define MCRESTOOL_GENEDITORWINDOW_H

#include <QWidget>

enum EditorStatus {
    EDITOR_STATUS_PERSISTENT,
    EDITOR_STATUS_CHANGED,
    EDITOR_STATUS_UNSAVED,
};

class GenEditorWindow : public QWidget {
Q_OBJECT

public:
    explicit GenEditorWindow(QWidget* parent);

    ~GenEditorWindow() override = default;

    virtual void save() = 0;

    virtual void reload() = 0;

    virtual bool pre_close();

    virtual EditorStatus status() = 0;

};

#endif //MCRESTOOL_GENEDITORWINDOW_H
