#ifndef MCRESTOOL_FSTREEMODEL_H
#define MCRESTOOL_FSTREEMODEL_H

#include <QAbstractItemModel>
#include <mcrtlib.h>

class FsTreeModel : public QAbstractItemModel, public mcrtlib::ffi::TreeChangeSubscriber {
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

    void pre_insert(const rust::Vec<size_t>& path, size_t start, size_t end) override;

    void post_insert(const rust::Vec<size_t>& path) override;

    void pre_remove(const rust::Vec<size_t>& path, size_t start, size_t end) override;

    void post_remove(const rust::Vec<size_t>& path) override;

private:
    QModelIndex find_path(const rust::Vec<size_t>& path);

    mcrtlib::ffi::Workspace& ws;

};

#endif //MCRESTOOL_FSTREEMODEL_H
