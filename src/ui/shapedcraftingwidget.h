#ifndef MCRESTOOL_SHAPEDCRAFTINGWIDGET_H
#define MCRESTOOL_SHAPEDCRAFTINGWIDGET_H

#include <QWidget>
#include "recipeeditextensionwidget.h"

namespace Ui {
    class ShapedCraftingWidget;
}

class ShapedCraftingWidget : public RecipeEditExtensionWidget {
    Q_OBJECT

public:
    explicit ShapedCraftingWidget(QWidget* parent = nullptr);

    ~ShapedCraftingWidget() override;

private:
    QScopedPointer<Ui::ShapedCraftingWidget> ui;

};

#endif //MCRESTOOL_SHAPEDCRAFTINGWIDGET_H
