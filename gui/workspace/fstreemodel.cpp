#include "fstreemodel.h"
#include <mcrtlib.h>
#include <mcrtutil.h>

using mcrtlib::ffi::Workspace;
using mcrtlib::ffi::WorkspaceRoot;
using mcrtlib::ffi::FsTreeEntry;
using mcrtlib::ffi::fstreeentry_from_ptr;
using mcrtlib::to_qstring;

FsTreeModel::FsTreeModel(Workspace& ws, QObject* parent) :
    QAbstractItemModel(parent),
    ws(ws) {}

FsTreeModel::~FsTreeModel() = default;

QModelIndex FsTreeModel::index(int row, int column, const QModelIndex& parent) const {
    if (!hasIndex(row, column, parent)) return QModelIndex();

    quintptr data;

    if (!parent.isValid()) {
        FsTreeEntry entry = this->ws.by_index(row).tree();
        assert(!entry.is_null1());
        data = entry.to_ptr();
    } else {
        FsTreeEntry entry = fstreeentry_from_ptr(parent.internalId());
        assert(!entry.is_null1());
        FsTreeEntry child = entry.by_index1(row);
        assert(!child.is_null1());
        data = child.to_ptr();
    }

    if (!data) {
        return QModelIndex();
    }

    return createIndex(row, column, data);
}

QModelIndex FsTreeModel::parent(const QModelIndex& child) const {
    if (!child.isValid()) return QModelIndex();

    FsTreeEntry entry = fstreeentry_from_ptr(child.internalId());
    assert(!entry.is_null1());
    if (entry.is_root()) {
        return QModelIndex();
    } else {
        FsTreeEntry parent = entry.parent();
        assert(!parent.is_null1());

        return createIndex((int) parent.index_of(entry), 0, parent.to_ptr());
    }
}

QVariant FsTreeModel::data(const QModelIndex& index, int role) const {
    if (!index.isValid()) return QVariant();
    if (role != Qt::DisplayRole) return QVariant();

    FsTreeEntry entry = fstreeentry_from_ptr(index.internalId());
    assert(!entry.is_null1());

    return to_qstring(entry.name());
}

Qt::ItemFlags FsTreeModel::flags(const QModelIndex& index) const {
    if (!index.isValid()) return Qt::NoItemFlags;

    return QAbstractItemModel::flags(index);
}

QVariant FsTreeModel::headerData(int section, Qt::Orientation orientation, int role) const {
    return QVariant();
}

int FsTreeModel::rowCount(const QModelIndex& parent) const {
    if (parent.column() > 0) return 0;

    FsTreeEntry entry = fstreeentry_from_ptr(parent.internalId());
    if (entry.is_null1()) {
        return this->ws.root_count();
    } else {
        return entry.children_count();
    }
}

int FsTreeModel::columnCount(const QModelIndex& parent) const {
    return 1;
}

QModelIndex FsTreeModel::find_path(const rust::Vec<size_t>& path) {
    QModelIndex model_index;

    for (auto index: path) {
        model_index = this->index(index, 0, model_index);

        if (!model_index.isValid()) {
            return QModelIndex();
        }
    }

    return model_index;
}

void FsTreeModel::pre_insert(const rust::Vec<size_t>& path, size_t start, size_t end) {
    QAbstractItemModel::beginInsertRows(find_path(path), (int) start, (int) end);
}

void FsTreeModel::post_insert(const rust::Vec<size_t>&) {
    QAbstractItemModel::endInsertRows();
}

void FsTreeModel::pre_remove(const rust::Vec<size_t>& path, size_t start, size_t end) {
    QAbstractItemModel::beginRemoveRows(find_path(path), (int) start, (int) end);
}

void FsTreeModel::post_remove(const rust::Vec<size_t>&) {
    QAbstractItemModel::endRemoveRows();
}

