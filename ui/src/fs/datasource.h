#ifndef MCRESTOOL_DATASOURCE_H
#define MCRESTOOL_DATASOURCE_H

#include <QIODevice>

class DataSource : public QObject {
Q_OBJECT

protected:
    explicit DataSource(QObject* parent = nullptr);

public:
    QIODevice* file(const QString& path);

    QStringList list_dir(const QString& path);

    bool delete_file(const QString& path);

    bool read_only();

};

#endif //MCRESTOOL_DATASOURCE_H
