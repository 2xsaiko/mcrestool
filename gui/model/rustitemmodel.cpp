#include "rustitemmodel.h"

RustItemModelBase::RustItemModelBase(QObject* parent) : QAbstractItemModel(parent) {
}

QModelIndex RustItemModelBase::find_path(const rust::Vec<size_t>& path) {
    QModelIndex model_index;

    for (auto index: path) {
        model_index = this->index(index, 0, model_index);

        if (!model_index.isValid()) {
            return QModelIndex();
        }
    }

    return model_index;
}

void RustItemModelBase::pre_insert(const rust::Vec<size_t>& path, size_t start, size_t end) {
    QAbstractItemModel::beginInsertRows(find_path(path), (int) start, (int) end);
}

void RustItemModelBase::post_insert(const rust::Vec<size_t>&) {
    QAbstractItemModel::endInsertRows();
}

void RustItemModelBase::pre_remove(const rust::Vec<size_t>& path, size_t start, size_t end) {
    QAbstractItemModel::beginRemoveRows(find_path(path), (int) start, (int) end);
}

void RustItemModelBase::post_remove(const rust::Vec<size_t>&) {
    QAbstractItemModel::endRemoveRows();
}
