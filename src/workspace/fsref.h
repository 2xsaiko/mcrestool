#ifndef MCRESTOOL_FSREF_H
#define MCRESTOOL_FSREF_H

#include "filetype.h"

#include <QFile>
#include <QList>
#include <QSharedPointer>
#include <quazip.h>
#include <path.h>

struct DirEntry;

struct NormalFsRef {
    QString file_path;
};

struct ZipFsRef {
    QString zip_path;
    QSharedPointer<QuaZip> qz;

    QString file_path;
};

enum FsRefType {
    FSREF_NORMAL,
    FSREF_ZIP
};

class FsRef {

public:
    explicit FsRef(const Path& file_path);

    explicit FsRef(const Path& zip_path, const Path& file_path);

    FsRef(const FsRef&);

    ~FsRef();

    [[nodiscard]] bool read_only() const;

    [[nodiscard]] bool is_file() const;

    [[nodiscard]] bool is_dir() const;

    [[nodiscard]] bool is_link() const;

    [[nodiscard]] QString file_name() const;

    [[nodiscard]] Path path() const;

    [[nodiscard]] QSharedPointer<QIODevice> open() const;

    [[nodiscard]] QList<DirEntry> read_dir() const;

    [[nodiscard]] QList<DirEntry> read_dir_recursive() const;

    bool remove(bool recursive) const;

    [[nodiscard]] FileType file_type() const;

    [[nodiscard]] FsRef parent() const;

    FsRef join(const Path& rel_path);

    [[nodiscard]] FsRefType type() const;

private:
    FsRefType m_type;
    union {
        NormalFsRef* m_normal;
        ZipFsRef* m_zip;
    };

};

#endif //MCRESTOOL_FSREF_H
