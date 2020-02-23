#ifndef MCRESTOOL_TREEITEM_H
#define MCRESTOOL_TREEITEM_H

#include <QVariant>

class TreeItem {
public:
    explicit TreeItem(const QString& text, TreeItem* parentItem = nullptr);

    ~TreeItem();

    void append_child(TreeItem* child);

    TreeItem* child(int row);

    int child_count() const;

    QString text() const;

    int row() const;

    TreeItem* parent_item();

private:
    QVector<TreeItem*> _child_items;
    QString _item_data;
    TreeItem* _parent_item;
};

#endif //MCRESTOOL_TREEITEM_H
