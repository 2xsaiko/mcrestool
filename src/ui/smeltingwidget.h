#ifndef MCRESTOOL_SMELTINGWIDGET_H
#define MCRESTOOL_SMELTINGWIDGET_H

#include <QWidget>
#include "recipeeditextensionwidget.h"

namespace Ui {
    class SmeltingWidget;
}

class SmeltingWidget : public RecipeEditExtensionWidget {
Q_OBJECT

public:
    explicit SmeltingWidget(QWidget* parent = nullptr);

    ~SmeltingWidget() override;

private:
    QScopedPointer<Ui::SmeltingWidget> ui;

};

#endif //MCRESTOOL_SMELTINGWIDGET_H
