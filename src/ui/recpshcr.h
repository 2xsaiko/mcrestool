#ifndef MCRESTOOL_RECPSHCR_H
#define MCRESTOOL_RECPSHCR_H

#include <QWidget>
#include "rcpedext.h"

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

#endif //MCRESTOOL_RECPSHCR_H
