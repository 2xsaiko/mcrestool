#ifndef MCRESTOOL_MAINWINDOW_H
#define MCRESTOOL_MAINWINDOW_H

#include <QMainWindow>
#include <QScopedPointer>
#include <QMdiSubWindow>
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

signals:

    void save_clicked();

    void save_all_clicked();

private slots:

    void quit();

    void open();

    void save();

    void save_as();

    void add_res_file();

    void add_res_folder();

    void show_resource_tree(bool shown);

    void show_game_objects(bool shown);

    void sub_window_focus_change(QMdiSubWindow* window);

private:
    QScopedPointer<Ui::MainWindow> ui;
    // ResourceTree rt;

};

#endif //MCRESTOOL_MAINWINDOW_H
