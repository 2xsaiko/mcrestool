#include "workspace.h"
#include <QFileInfo>

WorkspaceRootBase::WorkspaceRootBase(const QString& name, QObject* parent) : QObject(parent) {
    this->name = name;
}

const QString& WorkspaceRootBase::get_name() const {
    return this->name;
}

void WorkspaceRootBase::set_name(QString str) {
    this->name = str;
}


Workspace::Workspace(QObject* parent) : QObject(parent), roots(QList<WorkspaceRootBase*>()) {
}

void Workspace::add_dir(QString path) {
    this->roots += new DirWorkspaceRoot(path, this);
}

void Workspace::add_file(QString path) {
    this->roots += new ZipWorkspaceRoot(path);
}


DirWorkspaceRoot::DirWorkspaceRoot(const QString& path, QObject* parent) : WorkspaceRootBase(QFileInfo(path).fileName(), parent) {
    this->path = path;
}

QList<WSDirEntry> DirWorkspaceRoot::list_dir_tree(const QString& path) {
    QList<WSDirEntry> v;
    return v;
}


ZipWorkspaceRoot::ZipWorkspaceRoot(const QString& path, QObject* parent) : WorkspaceRootBase(QFileInfo(path).fileName(), parent) {
    this->path = path;
}

QList<WSDirEntry> ZipWorkspaceRoot::list_dir_tree(const QString& path) {
    QList<WSDirEntry> v;
    return v;
}
