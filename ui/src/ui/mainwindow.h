#ifndef MCRESTOOL_MAINWINDOW_H
#define MCRESTOOL_MAINWINDOW_H

#include <QMainWindow>
#include <QScopedPointer>
#include "src/model/resourcetree.h"
#include "src/model/languagetablemodel.h"

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

    void add_res_file();

    void add_res_folder();

    void show_resource_tree(bool shown);

    void show_game_objects(bool shown);

private:
    QScopedPointer<Ui::MainWindow> ui;
    // ResourceTree rt;

};

#endif //MCRESTOOL_MAINWINDOW_H
