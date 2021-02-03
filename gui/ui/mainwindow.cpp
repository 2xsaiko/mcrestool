#include "mainwindow.h"
#include "ui_mainwindow.h"

#include <QDesktopWidget>
#include <QScreen>
#include <QFileDialog>
#include <QInputDialog>
#include <QMessageBox>
#include <QDebug>

#include <fstreemodel.h>
#include "languagetablewindow.h"
#include "modeleditwindow.h"

using mcrtlib::ffi::FileType;
using mcrtlib::ffi::FsTreeEntry;
using mcrtlib::ffi::Workspace;
using mcrtlib::ffi::fstreeentry_from_ptr;
using mcrtlib::ffi::workspace_new;
using mcrtlib::ffi::workspace_from;
using mcrtlib::to_qstring;

MainWindow::MainWindow(QWidget* parent) : QMainWindow(parent),
                                          ui(new Ui::MainWindow),
                                          m_ws(workspace_new()),
                                          m_fstree_model(new FsTreeModel(this->m_ws, this)),
                                          m_ws_path(QString()) {
    ui->setupUi(this);

    connect(ui->action_quit, SIGNAL(triggered()), this, SLOT(quit()));
    connect(ui->action_open, SIGNAL(triggered()), this, SLOT(open()));
    connect(ui->action_save, SIGNAL(triggered()), this, SLOT(save()));
    connect(ui->action_open_workspace, SIGNAL(triggered()), this, SLOT(open_workspace()));
    connect(ui->action_save_workspace, SIGNAL(triggered()), this, SLOT(save_workspace()));
    connect(ui->action_close_workspace, SIGNAL(triggered()), this, SLOT(close_workspace()));
    connect(ui->action_about_qt, &QAction::triggered, &QApplication::aboutQt);

    connect(ui->action_new_block_model, SIGNAL(triggered()), this, SLOT(test_open_model_win()));

    connect(ui->action_resource_tree, SIGNAL(triggered(bool)), this, SLOT(show_resource_tree(bool)));
    connect(ui->res_tree, SIGNAL(visibilityChanged(bool)), this, SLOT(show_resource_tree(bool)));
    connect(ui->action_game_objects, SIGNAL(triggered(bool)), this, SLOT(show_game_objects(bool)));
    connect(ui->game_objects, SIGNAL(visibilityChanged(bool)), this, SLOT(show_game_objects(bool)));

    connect(ui->mdi_area, SIGNAL(subWindowActivated(QMdiSubWindow * )), this, SLOT(sub_window_focus_change(QMdiSubWindow * )));
    connect(ui->action_cascade, SIGNAL(triggered(bool)), this, SLOT(win_cascade()));
    connect(ui->action_tile, SIGNAL(triggered(bool)), this, SLOT(win_tile()));

    connect(ui->res_tree_view, SIGNAL(customContextMenuRequested(QPoint)), this, SLOT(show_restree_context_menu(QPoint)));
    connect(ui->res_tree_view, SIGNAL(activated(const QModelIndex &)), this, SLOT(restree_open(const QModelIndex &)));

    this->sub_window_focus_change(nullptr);
    ui->res_tree_view->setModel(this->m_fstree_model);
    this->m_ws.fst_subscribe(*this->m_fstree_model);
    // this->m_ws.gd_subscribe(*this->m_gameobject_model);
}

MainWindow::~MainWindow() {
    this->m_ws.fst_unsubscribe(*this->m_fstree_model);
    // this->m_ws.gd_unsubscribe(*this->m_gameobject_model);
}

void MainWindow::center() {
    QRect qRect = frameGeometry();
    const QScreen* screen = QGuiApplication::screenAt(QApplication::desktop()->cursor().pos());
    QPoint center = screen->geometry().center();
    qRect.moveCenter(center);
    this->move(qRect.topLeft());
}

void MainWindow::quit() {
    QApplication::quit();
}

void MainWindow::open() {

}

void MainWindow::save() {
    QMdiSubWindow* window = ui->mdi_area->activeSubWindow();
    if (window) {
        QWidget* widget = window->widget();
        auto* editorWindow = dynamic_cast<GenEditorWindow*>(widget);
        if (editorWindow) {
            editorWindow->save();
        } else {
            qDebug() << "Failed to save because" << editorWindow << "is not a GenEditorWindow!";
        }
    }
}

void MainWindow::open_workspace() {
    QString filename = QFileDialog::getOpenFileName(this, tr("Open Workspace"), QString(), "mcrestool Workspace(*.rtw)");
    if (!filename.isEmpty()) {
        if (!this->close_workspace()) return;
        std::string s = filename.toStdString();
        try {
            this->m_ws.from(s);
            this->m_ws_path = filename;
        } catch (const std::exception& e) {
            QMessageBox::critical(this, tr("Failed to Load Workspace"), tr("Failed to load workspace: %0").arg(e.what()));
            qDebug() << "failed to load workspace:" << e.what();
        }
    }
}

void MainWindow::save_workspace() {
    if (this->m_ws_path.isEmpty()) {
        this->save_workspace_as();
    } else {
        std::string s = this->m_ws_path.toStdString();
        this->m_ws.save(s);
    }
}

void MainWindow::save_workspace_as() {
    QString filename = QFileDialog::getSaveFileName(this, tr("Save Workspace"), QString(), "mcrestool Workspace(*.rtw)");
    if (!filename.isEmpty()) {
        std::string s = filename.toStdString();
        this->m_ws.save(s);
        this->m_ws_path = filename;
    }
}

bool MainWindow::close_workspace() {
    QList<QMdiSubWindow*> windows = ui->mdi_area->subWindowList();

    if (!windows.isEmpty()) {
        QMessageBox::StandardButton response = QMessageBox::question(
            this,
            tr("Close All Editors"),
            tr("Closing workspace. Do you want to also close all open editors?"),
            QMessageBox::Yes | QMessageBox::No | QMessageBox::Cancel,
            QMessageBox::StandardButton::Cancel);
        qDebug() << response;

        switch (response) {
            case QMessageBox::StandardButton::Yes:
                for (auto* window: windows) {
                    auto* editwin = qobject_cast<GenEditorWindow*>(window->widget());
                    if (!editwin || editwin->pre_close()) {
                        window->close();
                    } else {
                        // user canceled editor window close operation
                        return false;
                    }
                }
                break;
            case QMessageBox::StandardButton::No:
                break;
            case QMessageBox::StandardButton::Cancel:
            default:
                return false;
        }

    }

    this->m_ws.reset();
    return true;
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
    if (sources.isEmpty()) return;

    for (const auto& source: sources) {
        const std::string& path = source.toStdString();
        try {
            this->m_ws.add_zip(path);
        } catch (const std::exception& e) {
            qDebug() << "Failed to add path" << source << ":" << e.what();
        }
    }
}

void MainWindow::add_res_dir() {
    QString source = QFileDialog::getExistingDirectory(this, tr("Add Resource Folder"));
    if (source.isEmpty()) return;

    const std::string& path = source.toStdString();
    try {
        this->m_ws.add_dir(path);
    } catch (const std::exception& e) {
        qDebug() << "Failed to add path" << source << ":" << e.what();
    }
}

void MainWindow::detach_selected() {
    QModelIndexList indices = ui->res_tree_view->selectionModel()->selectedIndexes();

    for (const auto& index: indices) {
        FsTreeEntry entry = fstreeentry_from_ptr(index.internalId());
        if (entry.is_root()) {
            this->m_ws.detach(entry.root());
        }
    }
}

void MainWindow::close_selected() {
    QModelIndexList indices = ui->res_tree_view->selectionModel()->selectedIndexes();

    for (const auto& index: indices) {
        FsTreeEntry entry = fstreeentry_from_ptr(index.internalId());
        if (entry.is_root()) {
            this->m_ws.close(entry.root());
        }
    }
}

void MainWindow::open_selected() {
    QModelIndexList indices = ui->res_tree_view->selectionModel()->selectedIndexes();

    for (const auto& index: indices) {
        FsTreeEntry entry = fstreeentry_from_ptr(index.internalId());
        if (entry.is_root()) {
            this->m_ws.open(entry.root());
        }
    }
}

void MainWindow::sub_window_focus_change(QMdiSubWindow* window) {
    disconnect(ui->action_insert_language, &QAction::triggered, nullptr, nullptr);
    disconnect(ui->action_insert_translation_key, &QAction::triggered, nullptr, nullptr);
    ui->action_insert_language->setVisible(false);
    ui->action_insert_translation_key->setVisible(false);

    if (auto win = qobject_cast<LanguageTableWindow*>(window)) {
        connect(ui->action_insert_language, &QAction::triggered, win, &LanguageTableWindow::add_language);
        connect(ui->action_insert_translation_key, &QAction::triggered, win, &LanguageTableWindow::add_locale_key);
        ui->action_insert_language->setVisible(true);
        ui->action_insert_translation_key->setVisible(true);
    }
}

void MainWindow::show_restree_context_menu(const QPoint& pt) {
    QModelIndexList indices = ui->res_tree_view->selectionModel()->selectedIndexes();

    bool all_top_level = false;
    bool all_open = true;

    if (!indices.isEmpty()) {
        all_top_level = true;
        for (const auto& index: indices) {
            if (ui->res_tree_view->model()->parent(index) != QModelIndex()) {
                all_top_level = false;
                break;
            }

            if (!fstreeentry_from_ptr(index.internalId()).root().is_open()) {
                all_open = false;
            }
        }
    }

    const QPoint& gPt = ui->res_tree_view->mapToGlobal(pt);

    QMenu menu;
    menu.addAction(tr("&Add Directory"), this, SLOT(add_res_dir()))->setIcon(QIcon::fromTheme("document-import"));
    menu.addAction(tr("Add &ZIP File"), this, SLOT(add_res_file()))->setIcon(QIcon::fromTheme("document-import"));
    if (all_top_level) {
        menu.addSeparator();
        if (all_open) {
            menu.addAction(tr("&Close"), this, SLOT(close_selected()));
        } else {
            menu.addAction(tr("&Open"), this, SLOT(open_selected()));
        }
        menu.addAction(tr("&Detach"), this, SLOT(detach_selected()))->setIcon(QIcon::fromTheme("document-close"));
    }

    menu.exec(gPt);
}

void MainWindow::restree_open(const QModelIndex& index) {
    FsTreeEntry item = fstreeentry_from_ptr((size_t) index.internalPointer());
    if (!item.is_null()) {
        switch (item.file_type()) {
            case FileType::FILETYPE_LANGUAGE_PART:
            case FileType::FILETYPE_LANGUAGE: {
                auto* ltw = new LanguageTableWindow(new LanguageTableContainer(item.root().ds(), to_qstring(item.path()), this), this);
                ltw->reload();
                ui->mdi_area->addSubWindow(ltw);
                ltw->show();
                break;
            }
            default:
                qDebug() << "warning: unhandled file type" << (uint8_t) item.file_type();
        }
    }
}

void MainWindow::test_open_model_win() {
    ModelEditWindow* win = new ModelEditWindow(this);
    ui->mdi_area->addSubWindow(win);
    win->show();
}

void MainWindow::win_cascade() {
    ui->mdi_area->cascadeSubWindows();
}

void MainWindow::win_tile() {
    ui->mdi_area->tileSubWindows();
}
