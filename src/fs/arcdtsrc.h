#ifndef MCRESTOOL_ARCDTSRC_H
#define MCRESTOOL_ARCDTSRC_H

#include "datasrc.h"
#include <KArchive>

class ArchiveDataSource : public DataSource {
Q_OBJECT

public:
    explicit ArchiveDataSource(KArchive* archive, QObject* parent = nullptr);

    ~ArchiveDataSource() override;

    bool open(QIODevice::OpenMode mode) override;

    QIODevice* file(const QString& path) override;

    QStringList list_dir(const QString& path) override;

    bool read_only() override;

    void close() override;

    bool delete_file(const QString& path) override;

private:
    KArchive* archive;

    const KArchiveEntry* find_entry(const QString& path);
};

#endif //MCRESTOOL_ARCDTSRC_H
