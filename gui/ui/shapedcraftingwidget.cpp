#include "shapedcraftingwidget.h"
#include "ui_shapedcraftingwidget.h"

ShapedCraftingWidget::ShapedCraftingWidget(QWidget* parent) : RecipeEditExtensionWidget(parent), ui(new Ui::ShapedCraftingWidget) {
    ui->setupUi(this);
}

ShapedCraftingWidget::~ShapedCraftingWidget() = default;
