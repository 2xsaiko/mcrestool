#include "fsref.h"
#include "direntry.h"
#include <mcrtutil.h>

#include <QFileInfo>
#include <QDir>
#include <QDebug>
#include <quazipfile.h>

FsRef::FsRef(const Path& file_path) : m_type(FSREF_NORMAL), m_normal(new NormalFsRef) {
    this->m_normal->file_path = file_path.to_string();
}

FsRef::FsRef(const Path& zip_path, const Path& file_path) : m_type(FSREF_ZIP), m_zip(new ZipFsRef) {
    this->m_zip->zip_path = zip_path.to_string();
    this->m_zip->file_path = Path("/").join(file_path).to_string();
    this->m_zip->qz = QSharedPointer<QuaZip>(new QuaZip(zip_path.to_string()));
    this->m_zip->qz->open(QuaZip::mdUnzip);
}

FsRef::FsRef(const FsRef& that) : m_type(that.m_type) {
    switch (that.m_type) {
        case FSREF_NORMAL:
            this->m_normal = new NormalFsRef;
            this->m_normal->file_path = that.m_normal->file_path;
            break;
        case FSREF_ZIP:
            this->m_zip = new ZipFsRef;
            this->m_zip->zip_path = that.m_zip->zip_path;
            this->m_zip->qz = that.m_zip->qz;
            this->m_zip->file_path = that.m_zip->file_path;
            break;
    }
}

FsRef::~FsRef() {
    switch (this->m_type) {
        case FSREF_NORMAL:
            delete this->m_normal;
            break;
        case FSREF_ZIP:
            delete this->m_zip;
            break;
        default:
            unreachable();
    }
}

bool FsRef::read_only() const {
    switch (this->m_type) {
        case FSREF_NORMAL:
            return !QFileInfo(this->m_normal->file_path).isWritable();
        case FSREF_ZIP:
            return !QFileInfo(this->m_zip->zip_path).isWritable();
        default:
            unreachable();
    }
}

bool FsRef::is_file() const {
    switch (this->m_type) {
        case FSREF_NORMAL:
            return QFileInfo(this->m_normal->file_path).isFile();
        case FSREF_ZIP:
            return QuaZip(this->m_zip->zip_path).getFileNameList().contains(this->m_zip->file_path);
        default:
            unreachable();
    }
}

bool FsRef::is_dir() const {
    switch (this->m_type) {
        case FSREF_NORMAL:
            return QFileInfo(this->m_normal->file_path).isDir();
        case FSREF_ZIP:
            return !this->is_file();
        default:
            unreachable();
    }
}

bool FsRef::is_link() const {
    switch (this->m_type) {
        case FSREF_NORMAL:
            return !QFileInfo(this->m_normal->file_path).isSymbolicLink();
        case FSREF_ZIP:
            return false; // ain't no links in zip files
        default:
            unreachable();
    }
}

QString FsRef::file_name() const {
    return this->path().file_name();
}

Path FsRef::path() const {
    switch (this->m_type) {
        case FSREF_NORMAL:
            return this->m_normal->file_path;
        case FSREF_ZIP:
            return this->m_zip->file_path;
        default:
            unreachable();
    }
}

QSharedPointer<QIODevice> FsRef::open() const {
    switch (this->m_type) {
        case FSREF_NORMAL:
            return QSharedPointer<QIODevice>(new QFile(this->m_normal->file_path));
        case FSREF_ZIP:
            return QSharedPointer<QIODevice>(new QuaZipFile(this->m_zip->zip_path, this->m_zip->zip_path));
        default:
            unreachable();
    }
}

QList<DirEntry> FsRef::read_dir() const {
    qDebug() << "read_dir" << this->path().to_string();
    switch (this->m_type) {
        case FSREF_NORMAL: {
            QList<DirEntry> list;
            QDir dir(this->m_normal->file_path);
//            qDebug() << this->m_normal->file_path;
//            qDebug() << QFileInfo(this->m_normal->file_path).absoluteDir();

            for (const auto& entry : dir.entryInfoList(QDir::AllEntries | QDir::NoDotAndDotDot)) {
//                qDebug() << entry;
                list += DirEntry {
                    .is_file = entry.isFile(),
                    .is_dir = entry.isDir(),
                    .is_symlink = entry.isSymbolicLink(),
                    .path = entry.fileName(),
                    .ref = FsRef(entry.filePath()),
                };
            }

            return list;
        }
        case FSREF_ZIP: {
            QList<DirEntry> list;
            QSet<Path> scanned;
            QuaZip qz(this->m_zip->zip_path);
            qz.open(QuaZip::mdUnzip);
            if (qz.getFileNameList().contains(Path(this->m_zip->file_path).strip_prefix("/").to_string())) return list;

            Path cwd = "/";
            cwd.push(this->m_zip->file_path);

            for (const auto& entry : qz.getFileInfoList64()) {
                Path entry_path = "/";
                entry_path.push(entry.name);

                if (entry_path.starts_with(cwd)) {
                    Path file = entry_path.strip_prefix(cwd);
                    QString entry_name = file.components().next().to_string();

                    // entry_name is a directory if file has more than just the
                    // first path component (since then we have a file that's
                    // inside the entry_name directory)
                    bool is_dir = file.components().size() > 1;

                    // qDebug() << cwd.to_string() << entry_path.to_string() << file.to_string() << entry_name << is_dir;

                    if (!scanned.contains(entry_name)) {
                        if(is_dir)
//                        qDebug() << is_dir << entry_name;
                        list += DirEntry {
                            .is_file = !is_dir,
                            .is_dir = is_dir,
                            .is_symlink = false,
                            .path = entry_name,
                            .ref = FsRef(this->m_zip->zip_path, cwd.join(entry_name)),
                        };
                        scanned += entry_name;
                    }
                }
            }
            return list;
        }
        default:
            unreachable();
    }
}


QList<DirEntry> FsRef::read_dir_recursive() const {
    switch (this->m_type) {
        case FSREF_NORMAL: {
            QList<DirEntry> list;
            QDir dir(this->m_normal->file_path);
//            qDebug() << this->m_normal->file_path;
//            qDebug() << QFileInfo(this->m_normal->file_path).absoluteDir();

            for (const auto& entry : dir.entryInfoList(QDir::AllEntries | QDir::NoDotAndDotDot)) {
//                qDebug() << entry;
                list += DirEntry {
                    .is_file = entry.isFile(),
                    .is_dir = entry.isDir(),
                    .is_symlink = entry.isSymbolicLink(),
                    .path = entry.fileName(),
                    .ref = FsRef(entry.filePath()),
                };

                if (list.last().is_dir) {
                    list += list.last().ref.read_dir_recursive();
                }
            }

            return list;
        }
        case FSREF_ZIP: {
            QList<DirEntry> list;
            QSet<Path> dirs;
            QuaZip qz(this->m_zip->zip_path);
            qz.open(QuaZip::mdUnzip);
            if (qz.getFileNameList().contains(this->m_zip->file_path)) return list;

            Path cwd = "/";
            cwd.push(this->m_zip->file_path);

            QList<QuaZipFileInfo64> qList = qz.getFileInfoList64();

            for (const auto& entry : qList) {
                Path entry_path = "/";
                entry_path.push(entry.name);

//                qDebug() << cwd.to_string() << entry_path.to_string();

                if (entry_path.starts_with(cwd)) {
                    Path file = entry_path.strip_prefix(cwd);

                    Path path = entry_path.parent();
                    QList<Path> paths_to_create;

                    while (!path.is_empty() && path != cwd && !dirs.contains(path)) {
                        paths_to_create += path;
                        path = path.parent();
                    }

                    while (!paths_to_create.isEmpty()) {
                        Path p = paths_to_create.takeLast();
                        dirs += p;

                        qDebug() << p.to_string();

                        list += DirEntry {
                            .is_file = false,
                            .is_dir = true,
                            .is_symlink = false,
                            .path = p.strip_prefix(cwd),
                            .ref = FsRef(this->m_zip->zip_path, p),
                        };
                    }

                    list += DirEntry {
                        .is_file = true,
                        .is_dir = false,
                        .is_symlink = false,
                        .path = file,
                        .ref = FsRef(this->m_zip->zip_path, entry.name),
                    };
                }
            }

            std::sort(list.begin(), list.end(), [](const DirEntry& a, const DirEntry& b) { return (a.is_dir && !b.is_dir) || a.path.to_string() < b.path.to_string(); });


            return list;
        }
        default:
            unreachable();
    }
}

bool FsRef::remove(bool recursive) const {
    switch (this->m_type) {
        case FSREF_NORMAL: {
            QFileInfo fi(this->m_normal->file_path);
            if (fi.isFile()) {
                return QFile::remove(this->m_normal->file_path);
            } else if (fi.isDir()) {
                if (recursive) {
                    return QDir(this->m_normal->file_path).removeRecursively();
                } else {
                    // TODO does this remove directories? There's no QDir::remove for one path
                    return QFile::remove(this->m_normal->file_path);
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

FsRef FsRef::join(const Path& rel_path) {
    switch (this->m_type) {
        case FSREF_NORMAL:
            return FsRef(Path(this->m_normal->file_path).join(rel_path));
        case FSREF_ZIP:
            return FsRef(this->m_zip->zip_path, Path(this->m_zip->file_path).join(rel_path));
        default:
            unreachable();
    }
}

FsRef FsRef::parent() const {
    switch (this->m_type) {
        case FSREF_NORMAL: {
            auto dir = QDir(this->m_normal->file_path);
            dir.cdUp();
            return FsRef(dir.path());
        }
        case FSREF_ZIP:
            unimplemented();
        default:
            unreachable();
    }
}

FileType FsRef::file_type() const {
    // shitty detection for now
    if (this->path().extension() == "json" && this->parent().file_name() == "lang") {
        return FILETYPE_LANGUAGE_PART;
    } else if (this->file_name() == "lang") {
        return FILETYPE_LANGUAGE;
    }
    return FILETYPE_NONE;
}

FsRefType FsRef::type() const {
    return this->m_type;
}
