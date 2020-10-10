#include "fstree.h"
#include "workspace.h"

#include <QDebug>

FsTreeEntry::FsTreeEntry(FsRef ref, WorkspaceRoot* root, FsTreeEntry* parent) : QObject(parent), ref(ref), parent(parent), root(root) {

}

void FsTreeEntry::refresh() {
    this->type = this->ref.get_type();

    QList<DirEntry> list = this->ref.read_dir();

    bool changed = false;

    int i = 0;
    while (!list.isEmpty()) {
        DirEntry next = list[0];
        if (this->children.length() <= i) {
            this->children += new FsTreeEntry(next.real_path, this->root, this);
            changed = true;
        } else {
            QString name = this->children[i]->file_name();

            if (next.file_name != name) {
                while (this->children.length() > i && next.file_name > name) {
                    this->children.removeAt(i);
                }
                this->children.insert(i, new FsTreeEntry(next.real_path, this->root, this));
                changed = true;
            }
        }
        i++;
        list.pop_front();
    }

    while (this->children.length() > i) {
        this->children.removeLast();
    }

    if (changed) {
        emit children_changed();
    }

    for (auto c: this->children) {
        c->refresh();
    }
}

const FsRef& FsTreeEntry::fsref() const {
    return this->ref;
}

QString FsTreeEntry::file_name() const {
    return this->ref.file_name();
}

FileType FsTreeEntry::file_type() const {
    return this->type;
}

FsTreeEntry* FsTreeEntry::get_parent() {
    return this->parent;
}

WorkspaceRoot* FsTreeEntry::get_root() {
    return this->root;
}

int FsTreeEntry::children_count() const {
    return this->children.length();
}

int FsTreeEntry::index_of(FsTreeEntry* child) const {
    return this->children.indexOf(child);
}

FsTreeEntry* FsTreeEntry::by_index(int child) {
    return this->children[child];
}
