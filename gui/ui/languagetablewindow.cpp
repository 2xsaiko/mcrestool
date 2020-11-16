#include <QInputDialog>
#include "languagetablewindow.h"
#include "ui_languagetablewindow.h"
#include <mcrtutil.h>

LanguageTableWindow::LanguageTableWindow(LanguageTableContainer* ltc, QWidget* parent) : GenEditorWindow(parent),
                                                                                         ui(new Ui::LanguageTableWindow),
                                                                                         ltc(ltc) {
    ui->setupUi(this);
    const QString& string = ltc->path();
    this->setWindowTitle(this->windowTitle() + " - " + string);

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

void LanguageTableWindow::save() {
    ltc->save();
}

void LanguageTableWindow::reload() {
    ltc->load();
}

EditorStatus LanguageTableWindow::status() {
    if (!this->ltc->is_persistent()) {
        return EDITOR_STATUS_UNSAVED;
    } else if (this->ltc->is_changed()) {
        return EDITOR_STATUS_CHANGED;
    } else {
        return EDITOR_STATUS_PERSISTENT;
    }
}

LanguageTableWindow::~LanguageTableWindow() = default;
