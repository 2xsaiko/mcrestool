#include "mainwin.h"
#include "ui_mainwin.h"
#include "langtblw.h"
#include "recpedtw.h"
#include <QApplication>
#include <QDesktopWidget>
#include <QScreen>
#include <QFileDialog>
#include <QInputDialog>
#include <iostream>

MainWindow::MainWindow(QWidget* parent) : QMainWindow(parent), ui(new Ui::MainWindow) {
    ui->setupUi(this);

    connect(ui->action_quit, SIGNAL(triggered()), this, SLOT(quit()));
    connect(ui->action_open, SIGNAL(triggered()), this, SLOT(open()));
    connect(ui->action_save, SIGNAL(triggered()), this, SLOT(save()));
    connect(ui->action_save_as, SIGNAL(triggered()), this, SLOT(save_as()));
    connect(ui->action_resource_tree, SIGNAL(triggered(bool)), this, SLOT(show_resource_tree(bool)));
    connect(ui->res_tree, SIGNAL(visibilityChanged(bool)), this, SLOT(show_resource_tree(bool)));
    connect(ui->action_game_objects, SIGNAL(triggered(bool)), this, SLOT(show_game_objects(bool)));
    connect(ui->game_objects, SIGNAL(visibilityChanged(bool)), this, SLOT(show_game_objects(bool)));
    connect(ui->action_about_qt, &QAction::triggered, &QApplication::aboutQt);

    auto* ltw = new LanguageTableWindow(this);
    ui->mdi_area->addSubWindow(ltw);

    auto* crw = new RecipeEditWindow(this);
    ui->mdi_area->addSubWindow(crw);

    connect(ui->action_insert_language, &QAction::triggered, ltw, &LanguageTableWindow::add_language);
    connect(ui->action_insert_translation_key, &QAction::triggered, ltw, &LanguageTableWindow::add_locale_key);

    ui->res_tree_view->setModel(new ResourceTree(this));
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
    const QString& string = QFileDialog::getExistingDirectory(this, tr("Open Language File"), QString());
    if (!string.isNull()) {
        std::cout << "Selected " << string.toStdString() << '\n';
    }
}

void MainWindow::save() {

}

void MainWindow::save_as() {

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

MainWindow::~MainWindow() = default;
