#include <QDir>
#include "src/util.h"
#include "archivedatasource.h"

ArchiveDataSource::ArchiveDataSource(KArchive* archive, QObject* parent) :
    DataSource(parent),
    archive(archive) {}

ArchiveDataSource::~ArchiveDataSource() {
    delete archive;
}

const KArchiveEntry* ArchiveDataSource::find_entry(const QString& path) {
    QString clean_path = QDir::cleanPath("/" + path);
    QStringList parts = clean_path.split('/');
    const KArchiveDirectory* cd = archive->directory();
    const KArchiveEntry* entry = nullptr;

    auto iterator = parts.begin() + 1; // skip empty first path (to the left of the initial /)
    while (iterator != parts.end()) {
        entry = cd->entry(*iterator);
        if (!entry) return nullptr;

        if (entry->isDirectory()) {
            if (!(cd = dynamic_cast<const KArchiveDirectory*>(entry))) {
                unreachable();
            }
        }
        iterator += 1;
    }

    return entry;
}

bool ArchiveDataSource::delete_file(const QString& path) {
    return false;
}

QIODevice* ArchiveDataSource::file(const QString& path) {
    if (const auto* file = dynamic_cast<const KArchiveFile*>(find_entry(path))) {
        QIODevice* dev = file->createDevice();
        dev->setParent(this);
        return dev;
    }

    return nullptr;
}

QStringList ArchiveDataSource::list_dir(const QString& path) {
    if (const auto* dir = dynamic_cast<const KArchiveDirectory*>(find_entry(path))) {
        return dir->entries();
    }

    return QStringList();
}

bool ArchiveDataSource::read_only() {
    return true;
}

bool ArchiveDataSource::open(QIODevice::OpenMode mode) {
    return archive->open(mode);
}

void ArchiveDataSource::close() {
    archive->close();
}
