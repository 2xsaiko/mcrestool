#ifndef MCRESTOOL_MAINWIN_H
#define MCRESTOOL_MAINWIN_H

#include <QMainWindow>
#include <QScopedPointer>
#include <src/model/restree.h>
#include "src/model/langtbl.h"

namespace Ui {
    class MainWindow;
}

class MainWindow : public QMainWindow {
Q_OBJECT

public:
    explicit MainWindow(QWidget* parent = nullptr);

    ~MainWindow() override;

    void center();

private slots:

    void quit();

    void open();

    void save();

    void save_as();

    void show_resource_tree(bool shown);

    void show_game_objects(bool shown);

private:
    QScopedPointer<Ui::MainWindow> ui;
    // ResourceTree rt;

};

#endif // MCRESTOOL_MAINWIN_H
