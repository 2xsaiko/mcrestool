#ifndef MCRESTOOL_FSTREEMODEL_H
#define MCRESTOOL_FSTREEMODEL_H

#include "fstree.h"
#include "workspace.h"

#include <QAbstractItemModel>

class FsTreeModel : public QAbstractItemModel {
    Q_OBJECT

public:
    explicit FsTreeModel(Workspace* ws, QObject* parent = nullptr);

    QModelIndex index(int row, int column, const QModelIndex& parent) const override;

    QModelIndex parent(const QModelIndex& child) const override;

    QVariant data(const QModelIndex& index, int role) const override;

    Qt::ItemFlags flags(const QModelIndex& index) const override;

    QVariant headerData(int section, Qt::Orientation orientation, int role) const override;

    int rowCount(const QModelIndex& parent) const override;

    int columnCount(const QModelIndex& parent) const override;

public slots:

private:
    Workspace* ws;

};

#endif //MCRESTOOL_FSTREEMODEL_H
