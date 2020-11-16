#include "modeleditwindow.h"
#include "ui_modeleditwindow.h"

ModelEditWindow::ModelEditWindow(QWidget* parent) : QWidget(parent), ui(new Ui::ModelEditWindow) {
    ui->setupUi(this);
}

ModelEditWindow::~ModelEditWindow() = default;

void ModelEditWindow::save() {

}

void ModelEditWindow::reload() {

}

