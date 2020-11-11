#include "fstree.h"

#include <QDebug>
#include <utility>
#include "workspace.h"

using mcrtlib::ffi::DirEntry;
using mcrtlib::ffi::FileType;
using mcrtlib::to_qstring;

FsTreeEntry::FsTreeEntry(QString path, WorkspaceRoot* root, FsTreeEntry* parent) :
    QObject(parent),
    m_path(std::move(path)),
    m_type(FileType::FILETYPE_NONE),
    m_parent(parent),
    m_root(root) {

}

void FsTreeEntry::refresh() {
    const mcrtlib::ffi::DataSource& ds = this->m_root->ds();
    this->m_type = mcrtlib::get_file_type(ds, this->m_path);

    std::string path = this->m_path.toStdString();

    if (ds.is_dir(rust::Str(path))) {
        const rust::Vec<DirEntry>& vec = ds.list_dir(path);

        bool changed = false;
        int i = 0;

        for (const auto& entry: vec) {
            const QString& file_name = to_qstring(entry.file_name);
            if (this->m_children.length() <= i) {
                std::string s;
                this->m_children += new FsTreeEntry(this->m_path + "/" + file_name, this->m_root, this);
                changed = true;
            } else {
                QString name = this->m_children[i]->file_name();

                if (file_name != name) {
                    while (this->m_children.length() > i && file_name > name) {
                        this->m_children.removeAt(i);
                    }

                    this->m_children.insert(i, new FsTreeEntry(this->m_path + "/" + file_name, this->m_root, this));
                    changed = true;
                }
            }

            i++;
        }

        while (this->m_children.length() > i) {
            this->m_children.removeLast();
        }

        if (changed) {
            emit children_changed();
        }

        for (auto c: this->m_children) {
            c->refresh();
        }
    } else {
        // TODO: clear all children
    }
}

const QString& FsTreeEntry::path() const {
    return this->m_path;
}

QString FsTreeEntry::file_name() const {
    return this->m_path.mid(this->m_path.lastIndexOf("/") + 1);
}

FileType FsTreeEntry::file_type() const {
    return this->m_type;
}

FsTreeEntry* FsTreeEntry::parent() {
    return this->m_parent;
}

WorkspaceRoot* FsTreeEntry::root() {
    return this->m_root;
}

int FsTreeEntry::children_count() const {
    return this->m_children.length();
}

int FsTreeEntry::index_of(FsTreeEntry* child) const {
    return this->m_children.indexOf(child);
}

FsTreeEntry* FsTreeEntry::by_index(int child) {
    return this->m_children[child];
}

FsTreeEntry* FsTreeEntry::by_name(const QString& name) {
    for (FsTreeEntry* entry: this->m_children) {
        if (entry->file_name() == name) {
            return entry;
        }
    }

    return nullptr;
}
