#include "recpsmel.h"
#include "ui_recpsmel.h"

SmeltingWidget::SmeltingWidget(QWidget* parent) : RecipeEditExtensionWidget(parent), ui(new Ui::SmeltingWidget) {
    ui->setupUi(this);
}

SmeltingWidget::~SmeltingWidget() = default;
