#ifndef MCRESTOOL_RCPEDEXT_H
#define MCRESTOOL_RCPEDEXT_H

#include <QWidget>

class RecipeEditExtensionWidget : public QWidget {
Q_OBJECT

public:
    explicit RecipeEditExtensionWidget(QWidget* parent = nullptr);

    ~RecipeEditExtensionWidget() override;
};

#endif //MCRESTOOL_RCPEDEXT_H
