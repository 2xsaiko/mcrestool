#ifndef MCRESTOOL_FSTREEMODEL_H
#define MCRESTOOL_FSTREEMODEL_H

#include "fstree.h"
#include "workspace.h"

#include <QAbstractItemModel>

class FsTreeModel : public QAbstractItemModel {
    Q_OBJECT

public:
    explicit FsTreeModel(Workspace* ws, QObject* parent = nullptr);

    [[nodiscard]] QModelIndex index(int row, int column, const QModelIndex& parent) const override;

    [[nodiscard]] QModelIndex parent(const QModelIndex& child) const override;

    [[nodiscard]] QVariant data(const QModelIndex& index, int role) const override;

    [[nodiscard]] Qt::ItemFlags flags(const QModelIndex& index) const override;

    [[nodiscard]] QVariant headerData(int section, Qt::Orientation orientation, int role) const override;

    [[nodiscard]] int rowCount(const QModelIndex& parent) const override;

    [[nodiscard]] int columnCount(const QModelIndex& parent) const override;

    void beginInsertRows1(const QModelIndex &parent, int first, int last);

    void endInsertRows1();

public slots:

private:
    Workspace* ws;

};

#endif //MCRESTOOL_FSTREEMODEL_H
