#ifndef MCRESTOOL_RESTREE_H
#define MCRESTOOL_RESTREE_H

#include <QString>
#include <QAbstractItemModel>
#include "restritm.h"

class ResourceTree : public QAbstractItemModel {
Q_OBJECT

public:
    explicit ResourceTree(QObject* parent = nullptr);

    ~ResourceTree() override;

    QModelIndex index(int row, int column, const QModelIndex& parent) const override;

    QModelIndex parent(const QModelIndex& child) const override;

    QVariant data(const QModelIndex& index, int role) const override;

    Qt::ItemFlags flags(const QModelIndex& index) const override;

    QVariant headerData(int section, Qt::Orientation orientation, int role) const override;

    int rowCount(const QModelIndex& parent) const override;

    int columnCount(const QModelIndex& parent) const override;

private:
    TreeItem* root_item;

};

#endif //MCRESTOOL_RESTREE_H
