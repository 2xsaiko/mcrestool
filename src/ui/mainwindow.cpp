#include "mainwindow.h"
#include "ui_mainwindow.h"
#include "languagetablewindow.h"
#include "recipeeditwindow.h"
#include "src/workspace/fstreemodel.h"

#include <QDesktopWidget>
#include <QScreen>
#include <QFileDialog>
#include <QInputDialog>
#include <QDebug>

MainWindow::MainWindow(QWidget* parent) : QMainWindow(parent), ui(new Ui::MainWindow), ws(new Workspace(this)) {
    ui->setupUi(this);

    connect(ui->action_quit, SIGNAL(triggered()), this, SLOT(quit()));
    connect(ui->action_open, SIGNAL(triggered()), this, SLOT(open()));
    connect(ui->action_save, SIGNAL(triggered()), this, SLOT(save()));
    connect(ui->action_save_workspace_as, SIGNAL(triggered()), this, SLOT(save_as()));
    connect(ui->action_add_res_file, SIGNAL(triggered()), this, SLOT(add_res_file()));
    connect(ui->action_add_res_folder, SIGNAL(triggered()), this, SLOT(add_res_dir()));
    connect(ui->action_about_qt, &QAction::triggered, &QApplication::aboutQt);

    connect(ui->action_resource_tree, SIGNAL(triggered(bool)), this, SLOT(show_resource_tree(bool)));
    connect(ui->res_tree, SIGNAL(visibilityChanged(bool)), this, SLOT(show_resource_tree(bool)));
    connect(ui->action_game_objects, SIGNAL(triggered(bool)), this, SLOT(show_game_objects(bool)));
    connect(ui->game_objects, SIGNAL(visibilityChanged(bool)), this, SLOT(show_game_objects(bool)));

    connect(ui->mdi_area, SIGNAL(subWindowActivated(QMdiSubWindow * )), this, SLOT(sub_window_focus_change(QMdiSubWindow * )));

    auto* ltw = new LanguageTableWindow(new LanguageTableContainer(FsRef("../testres/assets/testmod/lang"), this), this);
    ltw->reload();
    ui->mdi_area->addSubWindow(ltw);

    // auto* ltw1 = new LanguageTableWindow(new LanguageTableContainer(FsRef("../testres/assets/minecraft/lang"), this), this);
    // ltw1->reload();
    // ui->mdi_area->addSubWindow(ltw1);

    auto* crw = new RecipeEditWindow(this);
    ui->mdi_area->addSubWindow(crw);

    connect(ui->action_insert_language, &QAction::triggered, ltw, &LanguageTableWindow::add_language);
    connect(ui->action_insert_translation_key, &QAction::triggered, ltw, &LanguageTableWindow::add_locale_key);

    connect(ui->res_tree_view, SIGNAL(customContextMenuRequested(QPoint)), this, SLOT(show_restree_context_menu(QPoint)));

    // ui->res_tree_view->setModel(new ResourceTree(this));
    ui->res_tree_view->setModel(new FsTreeModel(this->ws, this));
}

void MainWindow::center() {
    QRect qRect = frameGeometry();
    const QScreen* screen = QGuiApplication::screenAt(QApplication::desktop()->cursor().pos());
    const QPoint& center = screen->geometry().center();
    qRect.moveCenter(center);
    this->move(qRect.topLeft());
}

void MainWindow::quit() {
    QApplication::quit();
}

void MainWindow::open() {

}

void MainWindow::save() {
    QWidget* window = ui->mdi_area->activeSubWindow()->widget();
    GenEditorWindow* editorWindow = dynamic_cast<GenEditorWindow*>(window);
    if (editorWindow) {
        editorWindow->save();
    } else {
        qDebug() << "Failed to save because" << editorWindow << "is not a GenEditorWindow!";
    }
}

void MainWindow::save_as() {
    QString filename = QFileDialog::getSaveFileName(this, tr("Save Workspace"), QString(), "mcrestool Workspace(*.rtw)");
}

void MainWindow::show_resource_tree(bool shown) {
    ui->action_resource_tree->setChecked(shown);
    if (shown) {
        ui->res_tree->show();
    } else {
        ui->res_tree->hide();
    }
}

void MainWindow::show_game_objects(bool shown) {
    ui->action_game_objects->setChecked(shown);
    if (shown) {
        ui->game_objects->show();
    } else {
        ui->game_objects->hide();
    }
}

void MainWindow::add_res_file() {
    QStringList sources = QFileDialog::getOpenFileNames(this, tr("Add Resource Pack/Mod"), QString(), "Minecraft Content(*.zip *.jar);;All Files(*.*)");
    for (auto source: sources) {
        this->ws->add_file(source);
    }
}

void MainWindow::add_res_dir() {
    QString source = QFileDialog::getExistingDirectory(this, tr("Add Resource Folder"));
    this->ws->add_dir(source);
}

void MainWindow::sub_window_focus_change(QMdiSubWindow* window) {
    if (window) puts(window->widget()->objectName().toLocal8Bit());
}

void MainWindow::show_restree_context_menu(const QPoint& pt) {
    const QPoint& gPt = this->ui->res_tree_view->mapToGlobal(pt);

    QMenu menu;
    menu.addAction(tr("Add Directory"), this, SLOT(add_res_dir()));
    menu.addAction(tr("Add ZIP File"), this, SLOT(add_res_file()));

    menu.exec(gPt);
}

MainWindow::~MainWindow() = default;
