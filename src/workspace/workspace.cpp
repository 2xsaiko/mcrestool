#include "workspace.h"
#include "fstree.h"

WorkspaceRoot::WorkspaceRoot(QString name, FsRef root, QObject* parent) :
    QObject(parent),
    name(name),
    tree(new FsTreeEntry(root, this)) {
    this->tree->refresh();
}

FsTreeEntry* WorkspaceRoot::get_tree() {
    return this->tree;
}

const QString& WorkspaceRoot::get_name() const {
    return this->name;
}


Workspace::Workspace(QObject* parent) :
    QObject(parent),
    roots(QVector<WorkspaceRoot*>()) {}

void Workspace::add_dir(QString path) {
    this->roots += new WorkspaceRoot(path, FsRef(path), this);
    emit entry_added(this->roots.last());
}

void Workspace::add_file(QString path) {
    this->roots += new WorkspaceRoot(path, FsRef(path, "/"), this);
    emit entry_added(this->roots.last());
}

int Workspace::index_of(WorkspaceRoot* root) const {
    return this->roots.indexOf(root);
}

WorkspaceRoot* Workspace::by_index(int index) {
    if (index < 0 || index >= this->roots.length()) return nullptr;

    return this->roots[index];
}

int Workspace::root_count() const {
    return this->roots.length();
}
