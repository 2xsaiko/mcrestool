#ifndef MCRESTOOL_ITEMBTN_H
#define MCRESTOOL_ITEMBTN_H

#include <QtWidgets/QPushButton>
#include <src/identifr.h>

class ItemButton : public QPushButton {
Q_OBJECT

public:
    explicit ItemButton(QWidget* parent = nullptr);

    Identifier get_item();

    void set_item(Identifier id);

signals:

    void item_changed(Identifier item);

};

#endif //MCRESTOOL_ITEMBTN_H
