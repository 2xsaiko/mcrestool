#include "datasource.h"

DataSourceW::DataSourceW(DataSource* ds, QObject* parent) : QObject(parent), ds(ds) {}

DataSourceW::~DataSourceW() {
    datasource_delete(ds);
}

DataSourceW* DataSourceW::from_dir(const QString& dir, QObject* parent) {
    DataSource* p_source = datasource_dir_create(dir.toLocal8Bit());
    if (!p_source) {
        printf("error %d while trying to open datasource: %s\n", MCRT_ERROR, MCRT_ERROR_TEXT);
        return nullptr;
    }
    return new DataSourceW(p_source, parent);
}

DataSourceW* DataSourceW::from_zip(const QString& file, QObject* parent) {
    DataSource* p_source = datasource_zip_create(file.toLocal8Bit());
    if (!p_source) {
        return nullptr;
    }
    return new DataSourceW(p_source, parent);
}

bool DataSourceW::read_only() {
    return datasource_type(ds) == DATA_SOURCE_TYPE_ZIP;
}

ResFileW* DataSourceW::file(const QString& path) {
    return new ResFileW(this, path, this);
}

QStringList DataSourceW::list_dir(const QString& path) {
    QStringList list;
    const char* const* p_string = datasource_list_dir(ds, path.toLocal8Bit());
    while (p_string) {
        list += *p_string;
        p_string += 1;
    }
    dirlist_delete(p_string);
    return list;
}

bool DataSourceW::delete_file(const QString& path) {
    return datasource_delete_file(ds, path.toLocal8Bit());
}

DataSource* DataSourceW::inner() {
    return ds;
}
