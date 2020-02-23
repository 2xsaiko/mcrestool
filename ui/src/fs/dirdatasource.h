#ifndef MCRESTOOL_DIRDATASOURCE_H
#define MCRESTOOL_DIRDATASOURCE_H

#include "datasource.h"

class DirDataSource : public DataSource {
Q_OBJECT

public:
    explicit DirDataSource(const QString& path, QObject* parent = nullptr);

    QIODevice* file(const QString& path) override;

    bool open(QIODevice::OpenMode mode) override;

    void close() override;

    QStringList list_dir(const QString& path) override;

    bool delete_file(const QString& path) override;

private:
    QString path;

    bool read_only() override;

    QString get_file_path(const QString& path);

};

#endif //MCRESTOOL_DIRDATASOURCE_H
