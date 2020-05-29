#ifndef MCRESTOOL_DATASOURCE_H
#define MCRESTOOL_DATASOURCE_H

#include <QIODevice>
#include "mcrestool_logic.h"
#include "resfile.h"

class DataSourceW : public QObject {
Q_OBJECT

    friend class ResFileW;

private:
    explicit DataSourceW(DataSource* ds, QObject* parent = nullptr);

public:
    static DataSourceW* from_dir(const QString& dir, QObject* parent = nullptr);

    static DataSourceW* from_zip(const QString& file, QObject* parent = nullptr);

    ~DataSourceW() override;

    ResFileW* file(const QString& path);

    QStringList list_dir(const QString& path);

    bool delete_file(const QString& path);

    bool read_only();

    DataSource* inner();

private:
    DataSource* ds;

};

#endif //MCRESTOOL_DATASOURCE_H
