#include "fstreemodel.h"
#include <QDebug>
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

QModelIndex FsTreeModel::index(int row, int column, const QModelIndex& parent) const {
    if (!hasIndex(row, column, parent)) return QModelIndex();

    void* data;

    if (!parent.isValid()) {
        FsTreeEntry entry = this->ws.by_index_w(row).tree();
        assert(!entry.is_null_e());
        data = (void*) entry.to_ptr();
        qDebug() << "indexing from <invalid> to" << to_qstring(entry.path());
    } else {
        FsTreeEntry entry = fstreeentry_from_ptr((size_t) parent.internalPointer());
        assert(!entry.is_null_e());
        FsTreeEntry child = entry.by_index_e(row);
        assert(!child.is_null_e());
        data = (void*) child.to_ptr();
        qDebug() << "indexing from" << to_qstring(entry.path()) << "to" << to_qstring(child.path());
    }

    if (!data) {
        return QModelIndex();
    }

    return createIndex(row, column, data);
}

QModelIndex FsTreeModel::parent(const QModelIndex& child) const {
    if (!child.isValid()) return QModelIndex();

    FsTreeEntry entry = fstreeentry_from_ptr((size_t) child.internalPointer());
    assert(!entry.is_null_e());
    if (entry.is_root()) {
        qDebug() << "parent indexing from" << to_qstring(entry.path()) << "to <invalid>";
        return QModelIndex();
    } else {
        FsTreeEntry parent = entry.parent();
        assert(!parent.is_null_e());

        qDebug() << "parent indexing from" << to_qstring(entry.path()) << "to" << to_qstring(parent.path());

        return createIndex((int) parent.index_of(entry), 0, (void*) parent.to_ptr());
    }
}

QVariant FsTreeModel::data(const QModelIndex& index, int role) const {
    if (!index.isValid()) return QVariant();
    if (role != Qt::DisplayRole) return QVariant();

    FsTreeEntry entry = fstreeentry_from_ptr((size_t) index.internalPointer());
    assert(!entry.is_null_e());

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

    FsTreeEntry entry = fstreeentry_from_ptr((size_t) parent.internalPointer());
    if (entry.is_null_e()) {
        qDebug() << "root count:" << this->ws.root_count();
        return this->ws.root_count();
    } else {
        qDebug() << "children of " << to_qstring(entry.path()) << "count:" << entry.children_count();
        return entry.children_count();
    }
}

int FsTreeModel::columnCount(const QModelIndex& parent) const {
    return 1;
}

void FsTreeModel::beginInsertRows1(const QModelIndex& parent, int first, int last) {
    QAbstractItemModel::beginInsertRows(parent, first, last);
}

void FsTreeModel::endInsertRows1() {
    QAbstractItemModel::endInsertRows();
}

