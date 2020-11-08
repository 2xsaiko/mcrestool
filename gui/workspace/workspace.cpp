#include "workspace.h"
#include "fstree.h"

WorkspaceRoot::WorkspaceRoot(QString name, FsRef root, QObject* parent) :
    QObject(parent),
    m_name(name),
    m_tree(new FsTreeEntry(root, this)) {
    this->m_tree->refresh();
}

FsTreeEntry* WorkspaceRoot::tree() {
    return this->m_tree;
}

const QString& WorkspaceRoot::name() const {
    return this->m_name;
}


Workspace::Workspace(QObject* parent) :
    QObject(parent),
    m_roots(QVector<WorkspaceRoot*>()) {}

void Workspace::add_dir(QString path) {
    this->m_roots += new WorkspaceRoot(path, FsRef(path), this);
    emit entry_added(this->m_roots.last());
}

void Workspace::add_file(QString path) {
    this->m_roots += new WorkspaceRoot(path, FsRef(path, "/"), this);
    emit entry_added(this->m_roots.last());
}

int Workspace::index_of(WorkspaceRoot* root) const {
    return this->m_roots.indexOf(root);
}

WorkspaceRoot* Workspace::by_index(int index) {
    if (index < 0 || index >= this->m_roots.length()) return nullptr;

    return this->m_roots[index];
}

int Workspace::root_count() const {
    return this->m_roots.length();
}