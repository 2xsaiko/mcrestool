#ifndef MCRESTOOL_DATASOURCE_H
#define MCRESTOOL_DATASOURCE_H

#include <QIODevice>

class DataSource : public QObject {
Q_OBJECT

protected:
    explicit DataSource(QObject* parent = nullptr);

public:
    virtual QIODevice* file(const QString& path) = 0;

    virtual QStringList list_dir(const QString& path) = 0;

    virtual bool delete_file(const QString& path) = 0;

    virtual bool open(QIODevice::OpenMode mode) = 0;

    virtual void close() = 0;

    virtual bool read_only() = 0;

};

#endif //MCRESTOOL_DATASOURCE_H
