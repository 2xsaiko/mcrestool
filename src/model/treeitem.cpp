#include "treeitem.h"

TreeItem::TreeItem(const QString& data, TreeItem* parentItem) :
    _item_data(data), _parent_item(parentItem) {}

TreeItem::~TreeItem() {
    qDeleteAll(_child_items);
}

void TreeItem::append_child(TreeItem* child) {
    _child_items.append(child);
}

TreeItem* TreeItem::child(int row) {
    if (row < 0 || row >= _child_items.size()) return nullptr;
    return _child_items.at(row);
}

int TreeItem::child_count() const {
    return _child_items.size();
}

QString TreeItem::text() const {
    return _item_data;
}

int TreeItem::row() const {
    if (!_parent_item) return 0;
    return _parent_item->_child_items.indexOf(const_cast<TreeItem*>(this));
}

TreeItem* TreeItem::parent_item() {
    return _parent_item;
}
