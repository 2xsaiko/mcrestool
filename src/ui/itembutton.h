#ifndef MCRESTOOL_ITEMBUTTON_H
#define MCRESTOOL_ITEMBUTTON_H

#include <QPushButton>
#include <identifier.h>

class ItemButton : public QPushButton {
Q_OBJECT

public:
    explicit ItemButton(QWidget* parent = nullptr);

    Identifier get_item();

    void set_item(Identifier id);

signals:

    void item_changed(Identifier item);

};

#endif //MCRESTOOL_ITEMBUTTON_H
