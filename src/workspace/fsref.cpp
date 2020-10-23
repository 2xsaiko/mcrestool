#include "fsref.h"
#include "direntry.h"
#include "src/util.h"

#include <QFileInfo>
#include <QDir>
#include <QDebug>
#include <quazipfile.h>

FsRef::FsRef(const QString& file_path) : type(FSREF_NORMAL), normal(new NormalFsRef) {
    this->normal->file_path = file_path;
}

FsRef::FsRef(const QString& zip_path, const QString& file_path) : type(FSREF_ZIP), zip(new ZipFsRef) {
    this->zip->zip_path = zip_path;
    this->zip->file_path = file_path;
    this->zip->qz = QSharedPointer<QuaZip>(new QuaZip(zip_path));
    this->zip->qz->open(QuaZip::mdUnzip);
}

FsRef::FsRef(const FsRef& that) : type(that.type) {
    switch (that.type) {
        case FSREF_NORMAL:
            this->normal = new NormalFsRef;
            this->normal->file_path = that.normal->file_path;
            break;
        case FSREF_ZIP:
            this->zip = new ZipFsRef;
            this->zip->zip_path = that.zip->zip_path;
            this->zip->qz = that.zip->qz;
            this->zip->file_path = that.zip->file_path;
            break;
    }
}

FsRef::~FsRef() {
    switch (this->type) {
        case FSREF_NORMAL:
            delete this->normal;
            break;
        case FSREF_ZIP:
            delete this->zip;
            break;
        default:
            unreachable();
    }
}

bool FsRef::read_only() const {
    switch (this->type) {
        case FSREF_NORMAL:
            return !QFileInfo(this->normal->file_path).isWritable();
        case FSREF_ZIP:
            return !QFileInfo(this->zip->zip_path).isWritable();
        default:
            unreachable();
    }
}

bool FsRef::is_file() const {
    switch (this->type) {
        case FSREF_NORMAL:
            return QFileInfo(this->normal->file_path).isFile();
        case FSREF_ZIP:
            return QuaZip(this->zip->zip_path).getFileNameList().contains(this->zip->file_path);
        default:
            unreachable();
    }
}

bool FsRef::is_dir() const {
    switch (this->type) {
        case FSREF_NORMAL:
            return QFileInfo(this->normal->file_path).isDir();
        case FSREF_ZIP:
            return !this->is_file();
        default:
            unreachable();
    }
}

bool FsRef::is_link() const {
    switch (this->type) {
        case FSREF_NORMAL:
            return !QFileInfo(this->normal->file_path).isSymbolicLink();
        case FSREF_ZIP:
            return false; // ain't no links in zip files
        default:
            unreachable();
    }
}

QString FsRef::file_name() const {
    // TODO cache
    QStringRef file_name;
    int from = 0;
    int idx;

    QString file_path;

    switch (this->type) {
        case FSREF_NORMAL:
            file_path = this->normal->file_path;
            break;
        case FSREF_ZIP:
            file_path = this->zip->file_path;
            break;
        default:
            unreachable();
    }

    while (from < file_path.length()) {
        idx = file_path.indexOf('/', from);
        if (idx == -1) break;
        int next = file_path.indexOf('/', idx + 1);
        if (next == -1) next = file_path.length();
        if (next - idx - 1 >= 1) {
            file_name = QStringRef(&file_path, idx + 1, next - idx - 1);
        }
        from = next;
    }

    if (file_name.isNull()) return "<???>";

    return file_name.toString();
}

QSharedPointer<QIODevice> FsRef::open() const {
    switch (this->type) {
        case FSREF_NORMAL:
            return QSharedPointer<QIODevice>(new QFile(this->normal->file_path));
        case FSREF_ZIP:
            return QSharedPointer<QIODevice>(new QuaZipFile(this->zip->zip_path, this->zip->zip_path));
        default:
            unreachable();
    }
}

QList<DirEntry> FsRef::read_dir() const {
    switch (this->type) {
        case FSREF_NORMAL: {
            QList<DirEntry> list;
            QDir dir(this->normal->file_path);
            qDebug() << this->normal->file_path;
            qDebug() << QFileInfo(this->normal->file_path).absoluteDir();
            for (auto entry : dir.entryInfoList(QDir::AllEntries | QDir::NoDotAndDotDot)) {
                qDebug() << entry;
                list += DirEntry {
                    .is_file = entry.isFile(),
                    .is_dir = entry.isDir(),
                    .is_symlink = entry.isSymbolicLink(),
                    .file_name = entry.fileName(),
                    .real_path = FsRef(entry.filePath()),
                };
            }
            return list;
        }
        case FSREF_ZIP: {
            QList<DirEntry> list;
            QSet<QString> scanned;
            QuaZip qz(this->zip->zip_path);
            qz.open(QuaZip::mdUnzip);
            if (qz.getFileNameList().contains(this->zip->file_path)) return list;
            for (auto entry : qz.getFileInfoList64()) {
                QString prefix = this->zip->file_path;
                if (!prefix.endsWith('/')) prefix += '/';
                if (prefix == "/") prefix = "";
                qDebug() << prefix << entry.name;
                if (entry.name.startsWith(prefix)) {
                    int start = prefix.size();
                    int end = entry.name.indexOf('/', start);
                    bool is_dir = end != -1;
                    if (end == -1) end = entry.name.length();
                    QString name = entry.name.mid(start, end - start);
                    if (!scanned.contains(name)) {
                        qDebug() << is_dir << name;
                        list += DirEntry {
                            .is_file = !is_dir,
                            .is_dir = is_dir,
                            .is_symlink = false,
                            .file_name = name,
                            .real_path = FsRef(this->zip->zip_path, entry.name),
                        };
                        scanned += name;
                    }
                }
            }
            return list;
        }
        default:
            unreachable();
    }
}

bool FsRef::remove(bool recursive) const {
    switch (this->type) {
        case FSREF_NORMAL: {
            QFileInfo fi(this->normal->file_path);
            if (fi.isFile()) {
                return QFile::remove(this->normal->file_path);
            } else if (fi.isDir()) {
                if (recursive) {
                    return QDir(this->normal->file_path).removeRecursively();
                } else {
                    // TODO does this remove directories? There's no QDir::remove for one path
                    return QFile::remove(this->normal->file_path);
                }
            }
            return false;
        }
        case FSREF_ZIP:
            unimplemented();
        default:
            unreachable();
    }
}

FsRef FsRef::join(const QString& rel_path) {
    switch (this->type) {
        case FSREF_NORMAL:
            return FsRef(this->normal->file_path + "/" + rel_path);
        case FSREF_ZIP:
            return FsRef(this->zip->zip_path, this->zip->file_path + "/" + rel_path);
        default:
            unreachable();
    }
}

FsRef FsRef::parent() const {
    switch (this->type) {
        case FSREF_NORMAL: {
            auto dir = QDir(this->normal->file_path);
            dir.cdUp();
            return FsRef(dir.path());
        }
        case FSREF_ZIP:
            unimplemented();
        default:
            unreachable();
    }
}

FileType FsRef::get_type() const {
    // shitty detection for now
    if (this->file_name().endsWith(".json") && this->parent().file_name() == "lang") {
        return FILETYPE_LANGUAGE_PART;
    } else if (this->file_name() == "lang") {
        return FILETYPE_LANGUAGE;
    }
    return FILETYPE_NONE;
}
