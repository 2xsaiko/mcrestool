#ifndef MCRESTOOL_RECIPEEDITEXTENSIONWIDGET_H
#define MCRESTOOL_RECIPEEDITEXTENSIONWIDGET_H

#include <QWidget>

class RecipeEditExtensionWidget : public QWidget {
Q_OBJECT

public:
    explicit RecipeEditExtensionWidget(QWidget* parent = nullptr);

    ~RecipeEditExtensionWidget() override;
};

#endif //MCRESTOOL_RECIPEEDITEXTENSIONWIDGET_H
