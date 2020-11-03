#include <mcrtutil.h>
#include "recipeeditwindow.h"
#include "ui_recipeeditwindow.h"
#include "shapedcraftingwidget.h"
#include "smeltingwidget.h"

RecipeEditWindow::RecipeEditWindow(QWidget* parent) : QWidget(parent),
                                                      ui(new Ui::RecipeEditWindow),
                                                      extension_widget(nullptr) {
    ui->setupUi(this);

    connect(ui->type_box, SIGNAL(currentIndexChanged(int)), this, SLOT(change_recipe_type(int)));

    change_recipe_type(ui->type_box->currentIndex());
}

void RecipeEditWindow::change_recipe_type(RecipeType type) {
    delete extension_widget;

    if (type == SHAPED_CRAFTING || type == SHAPELESS_CRAFTING) {
        extension_widget = new ShapedCraftingWidget(this);
    } else if (type == SMELTING || type == CAMPFIRE || type == BLASTING || type == SMOKING) {
        extension_widget = new SmeltingWidget(this);
    } else {
        unimplemented();
    }

    ui->verticalLayout->addWidget(extension_widget);
}

RecipeEditWindow::~RecipeEditWindow() = default;
