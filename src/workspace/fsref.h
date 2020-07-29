#ifndef MCRESTOOL_FSREF_H
#define MCRESTOOL_FSREF_H

#include <QFile>
#include <QList>

struct WSDirEntry;

struct NormalFsRef {
    QString file_path;
};

struct ZipFsRef {
    QString zip_path;
    QString file_path;
};

enum FsRefType {
    NORMAL,
    ZIP,
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

    QIODevice* open() const;

    QList<WSDirEntry> read_dir() const;

private:
    FsRefType type;
    union {
        NormalFsRef normal;
        ZipFsRef zip;
    } data;

};

#endif //MCRESTOOL_FSREF_H
