#include <mcrtlib.h>
#include <lib.rs.h>

namespace mcrtlib {
    ffi::DataSource datasource_open(QString path) {
        const std::string& string = path.toStdString();
        return ffi::datasource_open(string);
    }

    ffi::DataSource datasource_open_zip(QString path) {
        const std::string& string = path.toStdString();
        return ffi::datasource_open_zip(string);
    }

    ffi::FileType get_file_type(const ffi::DataSource& ds, QString path) {
        const std::string& string = path.toStdString();
        return ffi::get_file_type(ds, string);
    }
}