#ifndef MCRESTOOL_LANGUAGETABLEWINDOW_H
#define MCRESTOOL_LANGUAGETABLEWINDOW_H

#include <QScopedPointer>
#include <QWidget>
#include "src/project/languagetablecontainer.h"

namespace Ui {
    class LanguageTableWindow;
}

class LanguageTableWindow : public QWidget {
Q_OBJECT

public:
    explicit LanguageTableWindow(LanguageTableContainer* ltc, QWidget* parent = nullptr);

    ~LanguageTableWindow() override;

    void add_language();

    void add_locale_key();

private:
    QScopedPointer<Ui::LanguageTableWindow> ui;
    LanguageTableContainer* ltc;

};

#endif //MCRESTOOL_LANGUAGETABLEWINDOW_H
