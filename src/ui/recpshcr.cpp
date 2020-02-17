#include "recpshcr.h"
#include "ui_recpshcr.h"

ShapedCraftingWidget::ShapedCraftingWidget(QWidget* parent) : RecipeEditExtensionWidget(parent), ui(new Ui::ShapedCraftingWidget) {
    ui->setupUi(this);
}

ShapedCraftingWidget::~ShapedCraftingWidget() = default;
