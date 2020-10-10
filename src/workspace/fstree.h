#ifndef MCRESTOOL_FSTREE_H
#define MCRESTOOL_FSTREE_H

#include "direntry.h"
#include "filetype.h"

#include <QList>

class WorkspaceRoot;

class FsTreeEntry : public QObject {
Q_OBJECT

public:
    explicit FsTreeEntry(FsRef ref, WorkspaceRoot* root, FsTreeEntry* parent = nullptr);

    void refresh();

    const FsRef& fsref() const;

    QString file_name() const;

    FileType file_type() const;

    FsTreeEntry* get_parent();

    WorkspaceRoot* get_root();

    int children_count() const;

    int index_of(FsTreeEntry* child) const;

    FsTreeEntry* by_index(int child);

signals:

    void children_changed();

private:
    FsRef ref;
    FileType type;

    FsTreeEntry* parent;
    QVector<FsTreeEntry*> children;

    WorkspaceRoot* root;

};

#endif //MCRESTOOL_FSTREE_H
