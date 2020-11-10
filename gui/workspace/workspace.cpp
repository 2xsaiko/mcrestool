#include "workspace.h"

#include <utility>
#include "fstree.h"

using mcrtlib::ffi::DataSource;

WorkspaceRoot::WorkspaceRoot(QString name, DataSource ds, QObject* parent) :
    QObject(parent),
    m_name(std::move(name)),
    m_ds(std::move(ds)),
    m_tree(new FsTreeEntry("/", this)) {
    this->m_tree->refresh();
}

FsTreeEntry* WorkspaceRoot::tree() {
    return this->m_tree;
}

const QString& WorkspaceRoot::name() const {
    return this->m_name;
}

const DataSource& WorkspaceRoot::ds() const {
    return this->m_ds;
}


Workspace::Workspace(QObject* parent) :
    QObject(parent),
    m_roots(QVector<WorkspaceRoot*>()) {}

void Workspace::add_dir(QString path) {
    DataSource ds = mcrtlib::datasource_open(path);
    this->m_roots += new WorkspaceRoot(path, ds, this);
    emit entry_added(this->m_roots.last());
}

void Workspace::add_file(QString path) {
    DataSource ds = mcrtlib::datasource_open_zip(path);
    this->m_roots += new WorkspaceRoot(path, ds, this);
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