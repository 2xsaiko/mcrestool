#ifndef MCRESTOOL_WORKSPACE_H
#define MCRESTOOL_WORKSPACE_H

#include <QString>
#include <mcrtlib.h>

#include "fstree.h"

class FsTreeEntry;

class WorkspaceRoot : public QObject {
    Q_OBJECT

public:
    WorkspaceRoot(QString name, mcrtlib::ffi::DataSource ds, QObject* parent = nullptr);

    FsTreeEntry* tree();

    [[nodiscard]] const QString& name() const;

    [[nodiscard]] const mcrtlib::ffi::DataSource& ds() const;

private:
    QString m_name;
    mcrtlib::ffi::DataSource m_ds;
    FsTreeEntry* m_tree;

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
    QVector<WorkspaceRoot*> m_roots;

};

#endif //MCRESTOOL_WORKSPACE_H
