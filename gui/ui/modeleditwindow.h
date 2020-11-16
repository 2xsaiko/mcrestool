#ifndef MCRESTOOL_MODELEDITWINDOW_H
#define MCRESTOOL_MODELEDITWINDOW_H

#include <QScopedPointer>
#include <QWidget>
#include "geneditorwindow.h"

namespace Ui {
    class ModelEditWindow;
}

class ModelEditWindow : public QWidget, public GenEditorWindow {
Q_OBJECT

public:
    explicit ModelEditWindow(QWidget* parent = nullptr);

    ~ModelEditWindow() override;

    void save() override;

    void reload() override;

private:
    QScopedPointer<Ui::ModelEditWindow> ui;

};

#endif //MCRESTOOL_MODELEDITWINDOW_H
