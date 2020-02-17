#ifndef MCRESTOOL_RECPEDTW_H
#define MCRESTOOL_RECPEDTW_H

#include <QtWidgets/QWidget>
#include "rcpedext.h"

namespace Ui {
    class RecipeEditWindow;
}

#define RecipeType int
#define SHAPED_CRAFTING 0
#define SHAPELESS_CRAFTING 1
#define SMELTING 2
#define CAMPFIRE 3
#define BLASTING 4
#define SMOKING 5
#define STONECUTTING 6

class RecipeEditWindow : public QWidget {
Q_OBJECT

public:
    explicit RecipeEditWindow(QWidget* parent = nullptr);

    ~RecipeEditWindow() override;

public slots:
    void change_recipe_type(RecipeType type);

private:
    QScopedPointer<Ui::RecipeEditWindow> ui;
    RecipeEditExtensionWidget* extension_widget;

};

#endif //MCRESTOOL_RECPEDTW_H
