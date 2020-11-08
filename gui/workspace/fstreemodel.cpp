#include "fstreemodel.h"
#include <mcrtutil.h>

FsTreeModel::FsTreeModel(Workspace* ws, QObject* parent) :
    QAbstractItemModel(parent),
    ws(ws) {}

QModelIndex FsTreeModel::index(int row, int column, const QModelIndex& parent) const {
    if (!hasIndex(row, column, parent)) return QModelIndex();

    void* data = nullptr;

    if (!parent.isValid()) {
        data = this->ws->by_index(row);
    } else {
        QObject* ptr = static_cast<QObject*>(parent.internalPointer());
        if (auto item = qobject_cast<WorkspaceRoot*>(ptr)) {
            data = item->tree()->by_index(row);
        } else if (auto item = qobject_cast<FsTreeEntry*>(ptr)) {
            data = item->by_index(row);
        }
    }

    if (!data) {
        return QModelIndex();
    }

    return createIndex(row, column, data);
}

QModelIndex FsTreeModel::parent(const QModelIndex& child) const {
    if (!child.isValid()) return QModelIndex();

    QObject* ptr = static_cast<QObject*>(child.internalPointer());
    if (auto item = qobject_cast<FsTreeEntry*>(ptr)) {
        FsTreeEntry* root = item->parent();
        assert(root != nullptr);

        if (root->parent() == nullptr) {
            // parent is top-level (WorkspaceRoot)
            return createIndex(root->index_of(item), 0, root->root());
        }

        return createIndex(root->index_of(item), 0, root);
    } else if (auto item = qobject_cast<WorkspaceRoot*>(ptr)) {
        return QModelIndex();
    } else {
        return QModelIndex();
    }
}

QVariant FsTreeModel::data(const QModelIndex& index, int role) const {
    if (!index.isValid()) return QVariant();
    if (role != Qt::DisplayRole) return QVariant();

    QObject* ptr = static_cast<QObject*>(index.internalPointer());
    if (auto item = qobject_cast<WorkspaceRoot*>(ptr)) {
        return item->name();
    } else if (auto item = qobject_cast<FsTreeEntry*>(ptr)) {
        return item->file_name();
    } else {
        return QVariant();
    }
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

    QObject* ptr = static_cast<QObject*>(parent.internalPointer());
    if (auto item = qobject_cast<FsTreeEntry*>(ptr)) {
        return item->children_count();
    } else if (auto item = qobject_cast<WorkspaceRoot*>(ptr)) {
        return item->tree()->children_count();
    } else {
        return this->ws->root_count();
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
