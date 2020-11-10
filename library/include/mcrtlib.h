#ifndef MCRESTOOL_MCRTLIB_H
#define MCRESTOOL_MCRTLIB_H

#include <QString>
#include <lib.rs.h>

#if defined(_MSC_VER)
#   define EXPORT __declspec(dllexport)
#   define IMPORT __declspec(dllimport)
#elif defined(__GNUC__)
#   define EXPORT __attribute__((visibility("default")))
#   define IMPORT
#else
#   define EXPORT
#   define IMPORT
#   pragma warning Unknown dynamic link import/export semantics.
#endif

#ifdef MCRTLIB_BUILD
#   define MCRTLIB_PUBLIC EXPORT
#else
#   define MCRTLIB_PUBLIC IMPORT
#endif

#define TO_QSTR(str) (QString::fromStdString(std::string(str)))
#define TO_RUST_STR(str) (rust::Str((str).toUtf8().constData()))

namespace mcrtlib {
    MCRTLIB_PUBLIC ffi::DataSource datasource_open(QString path);

    MCRTLIB_PUBLIC ffi::DataSource datasource_open_zip(QString path);

    MCRTLIB_PUBLIC ffi::FileType get_file_type(const ffi::DataSource& ds, QString path);
}

#endif //MCRESTOOL_MCRTLIB_H
