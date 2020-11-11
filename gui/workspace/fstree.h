#ifndef MCRESTOOL_FSTREE_H
#define MCRESTOOL_FSTREE_H

#include <QObject>
#include <QList>
#include <mcrtlib.h>

class WorkspaceRoot;

class FsTreeEntry : public QObject {
Q_OBJECT

public:
    explicit FsTreeEntry(QString  path, WorkspaceRoot* root, FsTreeEntry* parent = nullptr);

    void refresh();

    [[nodiscard]] const QString& path() const;

    [[nodiscard]] QString file_name() const;

    [[nodiscard]] mcrtlib::ffi::FileType file_type() const;

    FsTreeEntry* parent();

    WorkspaceRoot* root();

    int children_count() const;

    int index_of(FsTreeEntry* child) const;

    FsTreeEntry* by_index(int child);

    FsTreeEntry* by_name(const QString& name);

signals:

    void children_changed();

private:
    QString m_path;
    mcrtlib::ffi::FileType m_type;

    FsTreeEntry* m_parent;
    QVector<FsTreeEntry*> m_children;

    WorkspaceRoot* m_root;

};

#endif //MCRESTOOL_FSTREE_H
