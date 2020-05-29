#include <QMessageBox>
#include "mcrestool_logic.h"

void check_for_error(QWidget* parent) {
    if (MCRT_ERROR) {
        QMessageBox::critical(parent, "Error", QString("%2 (error %1)").arg(MCRT_ERROR).arg(MCRT_ERROR_TEXT));
    }
}
