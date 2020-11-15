#ifndef MCRESTOOL_MAINWINDOW_H
#define MCRESTOOL_MAINWINDOW_H

#include <QMainWindow>
#include <QScopedPointer>
#include <QMdiSubWindow>
#include <languagetablemodel.h>
#include <lib.rs.h>

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

    void add_res_dir();

    void show_resource_tree(bool shown);

    void show_game_objects(bool shown);

    void sub_window_focus_change(QMdiSubWindow* window);

    void show_restree_context_menu(const QPoint& pt);

    void restree_open(const QModelIndex& index);

private:
    QScopedPointer<Ui::MainWindow> ui;
    mcrtlib::ffi::Workspace ws;

};

#endif //MCRESTOOL_MAINWINDOW_H
