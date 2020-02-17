#include <QtWidgets/QInputDialog>
#include "langtblw.h"
#include "ui_langtblw.h"

LanguageTableWindow::LanguageTableWindow(QWidget* parent) : QWidget(parent), ui(new Ui::LanguageTableWindow) {
    ui->setupUi(this);

    t.add_language("en_us");

    ui->language_table_view->setModel(&t);
}

void LanguageTableWindow::add_language() {
    bool ok;
    QString text = QInputDialog::getText(this, tr("Insert Language…"), "Language:", QLineEdit::Normal, "", &ok);
    if (ok) {
        t.add_language(text);
    }
}

void LanguageTableWindow::add_locale_key() {
    bool ok;
    QString text = QInputDialog::getText(this, tr("Insert Localization Key…"), "Localization Key:", QLineEdit::Normal,
                                         "", &ok);
    if (ok) {
        t.add_locale_key(text);
    }
}

LanguageTableWindow::~LanguageTableWindow() = default;
