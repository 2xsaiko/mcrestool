#ifndef MCRESTOOL_FSTREEMODEL_H
#define MCRESTOOL_FSTREEMODEL_H

#include "rustitemmodel.h"
#include <mcrtlib.h>

class FsTreeModel : public RustItemModelBase {
    Q_OBJECT

public:
    explicit FsTreeModel(mcrtlib::ffi::Workspace& ws, QObject* parent = nullptr);

    ~FsTreeModel() override;

    [[nodiscard]] QModelIndex index(int row, int column, const QModelIndex& parent) const override;

    [[nodiscard]] QModelIndex parent(const QModelIndex& child) const override;

    [[nodiscard]] QVariant data(const QModelIndex& index, int role) const override;

    [[nodiscard]] Qt::ItemFlags flags(const QModelIndex& index) const override;

    [[nodiscard]] QVariant headerData(int section, Qt::Orientation orientation, int role) const override;

    [[nodiscard]] int rowCount(const QModelIndex& parent) const override;

    [[nodiscard]] int columnCount(const QModelIndex& parent) const override;

private:
    mcrtlib::ffi::Workspace& ws;

};

#endif //MCRESTOOL_FSTREEMODEL_H
