#include "fstreemodel.h"
#include <QIcon>
#include <mcrtlib.h>
#include <mcrtutil.h>

using mcrtlib::ffi::Workspace;
using mcrtlib::ffi::WorkspaceRoot;
using mcrtlib::ffi::FsTreeEntry;
using mcrtlib::ffi::fstreeentry_from_ptr;
using mcrtlib::to_qstring;
using rust::Str;

FsTreeModel::FsTreeModel(Workspace& ws, QObject* parent) :
    RustItemModelBase(parent),
    ws(ws) {}

FsTreeModel::~FsTreeModel() = default;

QModelIndex FsTreeModel::index(int row, int column, const QModelIndex& parent) const {
    if (!hasIndex(row, column, parent)) return QModelIndex();

    quintptr data;

    if (!parent.isValid()) {
        const WorkspaceRoot& root = this->ws.by_index(row);
        FsTreeEntry entry = root.tree();
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

    FsTreeEntry entry = fstreeentry_from_ptr(index.internalId());
    assert(!entry.is_null1());

    if (role == Qt::DisplayRole) {
        return to_qstring(entry.name());
    } else if (role == Qt::DecorationRole) {
        if (entry.is_root()) {
            if (entry.root().is_container_zip()) {
                return QIcon::fromTheme("application-zip");
            } else {
                return QIcon::fromTheme("folder-root");
            }
        }

        switch (entry.file_type()) {
            case mcrtlib::ffi::FileType::FILETYPE_LANGUAGE:
                return QIcon::fromTheme("folder-magenta");
            case mcrtlib::ffi::FileType::FILETYPE_LANGUAGE_PART:
            case mcrtlib::ffi::FileType::FILETYPE_RECIPE:
                return QIcon::fromTheme("application-json");
            default: {
                const rust::String& string = entry.path();
                if (entry.root().ds().is_dir(Str(string.data(), string.length()))) {
                    return QIcon::fromTheme("inode-directory");
                } else {
                    // TODO use a different icon because these files most likely
                    //      aren't zero-size
                    return QIcon::fromTheme("application-x-zerosize");
                }
            }
        }
    }

    return QVariant();
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