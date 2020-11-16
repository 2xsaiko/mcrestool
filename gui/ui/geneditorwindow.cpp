#include "geneditorwindow.h"

#include <QMessageBox>

GenEditorWindow::GenEditorWindow(QWidget* parent) : QWidget(parent) {

}

bool GenEditorWindow::pre_close() {
    if (this->status() != EDITOR_STATUS_PERSISTENT) {
        QMessageBox::StandardButton button = QMessageBox::warning(
            this,
            tr("Close File"),
            tr("The file %s has been modified. Do you want to save it before closing?"),
            QMessageBox::Yes | QMessageBox::No | QMessageBox::Cancel,
            QMessageBox::StandardButton::Cancel);

        if (button == QMessageBox::StandardButton::Yes) {
            this->save();
        }

        return button != QMessageBox::StandardButton::Cancel;
    } else {
        return true;
    }
}
