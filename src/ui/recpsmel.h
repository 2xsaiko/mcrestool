#ifndef MCRESTOOL_RECPSMEL_H
#define MCRESTOOL_RECPSMEL_H

#include <QWidget>
#include "rcpedext.h"

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

#endif //MCRESTOOL_RECPSMEL_H
