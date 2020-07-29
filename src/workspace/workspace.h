#ifndef MCRESTOOL_WORKSPACE_H
#define MCRESTOOL_WORKSPACE_H

#include "direntry.h"

#include <QString>

class WorkspaceRootBase : public QObject {
Q_OBJECT

public:
    WorkspaceRootBase(const QString& name, QObject* parent = nullptr);

    virtual ~WorkspaceRootBase() = default;

    virtual QList<WSDirEntry> list_dir_tree(const QString& path) = 0;

    const QString& get_name() const;

    void set_name(QString name);

private:
    QString name;

};

class DirWorkspaceRoot : public WorkspaceRootBase {

public:
    DirWorkspaceRoot(const QString& path, QObject* parent = nullptr);

    QList<WSDirEntry> list_dir_tree(const QString& path) override;

private:
    QString path;

};

class ZipWorkspaceRoot : public WorkspaceRootBase {

public:
    ZipWorkspaceRoot(const QString& path, QObject* parent = nullptr);

    QList<WSDirEntry> list_dir_tree(const QString& path) override;

private:
    QString path;

};

class Workspace : public QObject {
Q_OBJECT

public:
    Workspace(QObject* parent = nullptr);

    void add_dir(QString path);

    void add_file(QString path);

private:
    QList<WorkspaceRootBase*> roots;

};

#endif //MCRESTOOL_WORKSPACE_H
