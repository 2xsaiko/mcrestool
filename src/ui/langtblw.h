#ifndef MCRESTOOL_LANGTBLW_H
#define MCRESTOOL_LANGTBLW_H

#include <QScopedPointer>
#include <QtWidgets/QWidget>
#include "src/model/langtbl.h"

namespace Ui {
    class LanguageTableWindow;
}

class LanguageTableWindow : public QWidget {
Q_OBJECT

public:
    explicit LanguageTableWindow(QWidget* parent = nullptr);

    ~LanguageTableWindow() override;

    void add_language();

    void add_locale_key();

private:
    QScopedPointer<Ui::LanguageTableWindow> ui;
    LanguageTable t;

};

#endif //MCRESTOOL_LANGTBLW_H
