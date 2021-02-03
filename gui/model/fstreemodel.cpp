#include "fstreemodel.h"
#include <QIcon>
#include <QDebug>
#include <mcrtlib.h>
#include <mcrtutil.h>

using mcrtlib::ffi::Workspace;
using mcrtlib::ffi::FsTreeRoot;
using mcrtlib::ffi::FsTreeEntry;
using mcrtlib::ffi::fstreeentry_from_ptr;
using mcrtlib::to_qstring;
using rust::Str;

FsTreeModel::FsTreeModel(Workspace& ws, QObject* parent) :
    RustItemModel(parent),
    ws(ws) {}

FsTreeModel::~FsTreeModel() = default;


FsTreeEntry FsTreeModel::get_data(quintptr ptr) const {
    return fstreeentry_from_ptr(ptr);
}

QVariant FsTreeModel::get_display(const FsTreeEntry& data, int role) const {
    if (role == Qt::DisplayRole) {
        return to_qstring(data.name());
    } else if (role == Qt::DecorationRole) {
        if (data.is_root()) {
            if (data.root().is_container_zip()) {
                return QIcon::fromTheme("application-zip");
            } else {
                return QIcon::fromTheme("folder-root");
            }
        }

        switch (data.file_type()) {
            case mcrtlib::ffi::FileType::FILETYPE_LANGUAGE:
                return QIcon::fromTheme("folder-magenta");
            case mcrtlib::ffi::FileType::FILETYPE_LANGUAGE_PART:
            case mcrtlib::ffi::FileType::FILETYPE_RECIPE:
                return QIcon::fromTheme("application-json");
            default: {
                const rust::String& string = data.path();
                if (data.root().ds().is_dir(Str(string.data(), string.length()))) {
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

std::optional<FsTreeEntry> FsTreeModel::get_parent(const FsTreeEntry& data) const {
    if (data.is_root()) {
        return std::optional<FsTreeEntry>();
    } else {
        return data.parent();
    }
}

size_t FsTreeModel::children_count(optional_ref<const FsTreeEntry> data) const {
    if (data) {
        return data->get().children_count();
    } else {
        return this->ws.root_count();
    }
}

FsTreeEntry FsTreeModel::index(optional_ref<const FsTreeEntry> data, size_t row) const {
    if (data) {
        return data->get().by_index(row);
    } else {
        return this->ws.by_index(row).tree();
    }
}

size_t FsTreeModel::index_of(const FsTreeEntry& data) const {
    const auto& parent = this->get_parent(data);
    if (parent) {
        return parent->index_of(data);
    } else {
        return this->ws.index_of(data.root());
    }
}
