#include "fsref.h"
#include "direntry.h"
#include "src/util.h"

#include <QFileInfo>
#include <QDir>
#include <quazip5/quazip.h>
#include <quazip5/quazipfile.h>

FsRef::FsRef(const QString& file_path) : type(FsRefType::NORMAL) {
    data.normal = NormalFsRef {
        .file_path = file_path
    };
}

FsRef::FsRef(const QString& zip_path, const QString& file_path) : type(FsRefType::ZIP) {
    data.zip = ZipFsRef {
        .zip_path = zip_path,
        .file_path = file_path,
    };
}

FsRef::FsRef(const FsRef& that) : type(that.type) {
    switch(that.type) {
        case NORMAL:
            this->data.normal = that.data.normal;
            break;
        case ZIP:
            this->data.zip = that.data.zip;
            break;
        default:
            unreachable();
    }
}

FsRef::~FsRef() {
    switch (this->type) {
        case NORMAL:
            delete this->data.normal;
            break;
        case ZIP:
            delete this->data.zip;
            break;
        default:
            unreachable();
    }
}

bool FsRef::read_only() const {
    switch (this->type) {
        case NORMAL:
            return !QFileInfo(this->data.normal.file_path).isWritable();
        case ZIP:
            return !QFileInfo(this->data.zip.zip_path).isWritable();
        default:
            unreachable();
    }
}

bool FsRef::is_file() const {
    switch (this->type) {
        case NORMAL:
            return QFileInfo(this->data.normal.file_path).isFile();
        case ZIP:
            return QuaZip(this->data.zip.zip_path).getFileNameList().contains(this->data.zip.file_path);
        default:
            unreachable();
    }
}

bool FsRef::is_dir() const {
    switch (this->type) {
        case NORMAL:
            return QFileInfo(this->data.normal.file_path).isDir();
        case ZIP:
            // return !this->is_file();
            return !this->read_dir().isEmpty();
        default:
            unreachable();
    }
}

bool FsRef::is_link() const {
    switch (this->type) {
        case NORMAL:
            return !QFileInfo(this->data.normal.file_path).isSymbolicLink();
        case ZIP:
            return false; // ain't no links in zip files
        default:
            unreachable();
    }
}

QIODevice* FsRef::open() const {
    switch (this->type) {
        case NORMAL:
            return new QFile(this->data.normal.file_path);
        case ZIP:
            return new QuaZipFile(this->data.zip.zip_path, this->data.zip.file_path);
        default:
            unreachable();
    }
}

QList<WSDirEntry> FsRef::read_dir() const {
    switch (this->type) {
        case NORMAL: {
            QList<WSDirEntry> list;
            QDir dir(this->data.normal.file_path);
            for (auto entry : dir.entryInfoList()) {
                list += WSDirEntry {
                    .is_file = entry.isFile(),
                    .is_symlink = entry.isSymbolicLink(),
                    .is_dir = entry.isDir(),
                    .file_name = entry.fileName(),
                    .real_path = FsRef(entry.filePath()),
                };
            }
            return list;
        }
        case ZIP: {
            QList<WSDirEntry> list;
            QuaZipFile qzf(this->data.zip.zip_path, this->data.zip.file_path);
            QuaZip qz(this->data.zip.zip_path);
            for (auto entry : qz.getFileInfoList()) {
                QString prefix = this->data.zip.file_path;
                if (!prefix.endsWith('/')) prefix += '/';
                if (entry.name.startsWith(prefix)) {
                    int start = prefix.size();
                    int end = entry.name.indexOf('/', start);
                    bool is_dir = end != -1;
                    if (end == -1) end = entry.name.length();
                    QString name = entry.name.mid(start, end - start);
                    list += WSDirEntry {
                        .is_file = !is_dir,
                        .is_dir = is_dir,
                        .is_symlink = false,
                        .file_name = name,
                        .real_path = FsRef(this->data.zip.zip_path, entry.name),
                    };
                }
            }
        }
        default:
            unreachable();
    }
}
