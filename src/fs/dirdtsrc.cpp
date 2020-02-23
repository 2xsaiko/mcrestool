#include <QFile>
#include <QDir>
#include "dirdtsrc.h"

DirDataSource::DirDataSource(const QString& path, QObject* parent) : DataSource(parent), path(QDir::cleanPath(path)) {}

QIODevice* DirDataSource::file(const QString& path) {
    return new QFile(get_file_path(path), this);
}

QStringList DirDataSource::list_dir(const QString& path) {
    return QDir(QDir::cleanPath("/" + path)).entryList();
}

bool DirDataSource::read_only() {
    return false;
}

bool DirDataSource::open(QIODevice::OpenMode) {
    return QDir(path).exists();
}

void DirDataSource::close() {}

bool DirDataSource::delete_file(const QString& path) {
    return QFile::remove(get_file_path(path));
}

QString DirDataSource::get_file_path(const QString& path) {
    return this->path + QDir::cleanPath("/" + path);
}
