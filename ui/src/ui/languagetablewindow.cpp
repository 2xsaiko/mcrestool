#include <QInputDialog>
#include "languagetablewindow.h"
#include "ui_languagetablewindow.h"

LanguageTableWindow::LanguageTableWindow(LanguageTableContainer* ltc, QWidget* parent) : QWidget(parent),
                                                                                         ui(new Ui::LanguageTableWindow),
                                                                                         ltc(ltc) {
    ui->setupUi(this);

    ltc->language_table()->add_language("en_us");

    ui->language_table_view->setModel(ltc->language_table());
}

void LanguageTableWindow::add_language() {
    bool ok;
    QString text = QInputDialog::getText(this, tr("Insert Language…"), "Language:", QLineEdit::Normal, "", &ok);
    if (ok) {
        ltc->language_table()->add_language(text);
    }
}

void LanguageTableWindow::add_locale_key() {
    bool ok;
    QString text = QInputDialog::getText(this, tr("Insert Localization Key…"), "Localization Key:", QLineEdit::Normal,
                                         "", &ok);
    if (ok) {
        ltc->language_table()->add_locale_key(text);
    }
}

LanguageTableWindow::~LanguageTableWindow() = default;
