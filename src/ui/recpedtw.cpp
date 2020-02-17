#include "src/util.h"
#include "recpedtw.h"
#include "ui_recpedtw.h"
#include "recpshcr.h"

RecipeEditWindow::RecipeEditWindow(QWidget* parent) : QWidget(parent), ui(new Ui::RecipeEditWindow) {
    ui->setupUi(this);

    connect(ui->type_box, SIGNAL(currentIndexChanged(int)), this, SLOT(change_recipe_type(int)));

    change_recipe_type(ui->type_box->currentIndex());
}

void RecipeEditWindow::change_recipe_type(RecipeType type) {
    delete extension_widget;

    if (type == SHAPED_CRAFTING) {
        extension_widget = new ShapedCraftingWidget(this);
    } else {
        unimplemented();
    }

    ui->verticalLayout->addWidget(extension_widget);
}

RecipeEditWindow::~RecipeEditWindow() = default;
