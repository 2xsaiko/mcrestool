#include "smeltingwidget.h"
#include "ui_smeltingwidget.h"

SmeltingWidget::SmeltingWidget(QWidget* parent) : RecipeEditExtensionWidget(parent), ui(new Ui::SmeltingWidget) {
    ui->setupUi(this);
}

SmeltingWidget::~SmeltingWidget() = default;
