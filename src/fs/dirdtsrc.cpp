#include <QFile>
#include <QDir>
#include "dirdtsrc.h"

DirDataSource::DirDataSource(const QString& path, QObject* parent) : DataSource(parent), path(QDir::cleanPath(path)) {}

QIODevice* DirDataSource::file(const QString& path) {
    return new QFile(this->path + QDir::cleanPath("/" + path), this);
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
