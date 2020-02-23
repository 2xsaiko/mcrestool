#include "itembutton.h"

ItemButton::ItemButton(QWidget* parent) : QPushButton(parent) {
    setText("item!");
}

Identifier ItemButton::get_item() {
    return Identifier("minecraft:air");
}

void ItemButton::set_item(Identifier id) {
    emit item_changed(id);
}
