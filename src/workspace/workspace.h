#ifndef MCRESTOOL_WORKSPACE_H
#define MCRESTOOL_WORKSPACE_H

#include "direntry.h"

#include <QString>

class FsTreeEntry;

class WorkspaceRoot : public QObject {
    Q_OBJECT

public:
    WorkspaceRoot(QString name, FsRef root, QObject* parent = nullptr);

    FsTreeEntry* get_tree();

    const QString& get_name() const;

private:
    QString name;
    FsTreeEntry* tree;

};

class Workspace : public QObject {
Q_OBJECT

public:
    Workspace(QObject* parent = nullptr);

    void add_dir(QString path);

    void add_file(QString path);

    int index_of(WorkspaceRoot* root) const;

    WorkspaceRoot* by_index(int index);

    int root_count() const;

signals:

    void entry_added(WorkspaceRoot* root);

    void entry_removed(WorkspaceRoot* root);

private:
    QVector<WorkspaceRoot*> roots;

};

#endif //MCRESTOOL_WORKSPACE_H
