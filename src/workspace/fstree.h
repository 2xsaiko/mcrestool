#ifndef MCRESTOOL_FSTREE_H
#define MCRESTOOL_FSTREE_H

#include "direntry.h"
#include "filetype.h"

#include <QList>

class WorkspaceRoot;

class FsTreeEntry : public QObject {
Q_OBJECT

public:
    explicit FsTreeEntry(const FsRef& ref, WorkspaceRoot* root, FsTreeEntry* parent = nullptr);

    void refresh();

    const FsRef& ref() const;

    QString file_name() const;

    FileType file_type() const;

    FsTreeEntry* parent();

    WorkspaceRoot* root();

    int children_count() const;

    int index_of(FsTreeEntry* child) const;

    FsTreeEntry* by_index(int child);

    FsTreeEntry* by_name(const QString& name);

private:
    void refresh_as_zip();

    void refresh_as_normal();

signals:

    void children_changed();

private:
    FsRef m_ref;
    FileType m_type;

    FsTreeEntry* m_parent;
    QVector<FsTreeEntry*> m_children;

    WorkspaceRoot* m_root;

};

#endif //MCRESTOOL_FSTREE_H
