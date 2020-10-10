#ifndef MCRESTOOL_FSREF_H
#define MCRESTOOL_FSREF_H

#include "filetype.h"

#include <QFile>
#include <QList>
#include <QSharedPointer>
#include <quazip5/quazip.h>

struct DirEntry;

struct NormalFsRef {
    QString file_path;
};

struct ZipFsRef {
    QString zip_path;
    QSharedPointer<QuaZip> qz;

    QString file_path;
};

class FsRef {

public:
    explicit FsRef(const QString& file_path);

    explicit FsRef(const QString& zip_path, const QString& file_path);

    FsRef(const FsRef&);

    ~FsRef();

    bool read_only() const;

    bool is_file() const;

    bool is_dir() const;

    bool is_link() const;

    QString file_name() const;

    QSharedPointer<QIODevice> open() const;

    QList<DirEntry> read_dir() const;

    bool remove(bool recursive) const;

    FileType get_type() const;

    FsRef parent() const;

    FsRef join(const QString& rel_path);

private:
    enum { FSREF_NORMAL, FSREF_ZIP } type;
    union {
        NormalFsRef* normal;
        ZipFsRef* zip;
    };

};

#endif //MCRESTOOL_FSREF_H
