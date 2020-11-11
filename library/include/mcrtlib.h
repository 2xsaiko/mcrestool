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

namespace mcrtlib {
    MCRTLIB_PUBLIC ffi::DataSource datasource_open(QString path);

    MCRTLIB_PUBLIC ffi::DataSource datasource_open_zip(QString path);

    MCRTLIB_PUBLIC ffi::FileType get_file_type(const ffi::DataSource& ds, QString path);

    MCRTLIB_PUBLIC QString to_qstring(const rust::Str& str);

    MCRTLIB_PUBLIC QString to_qstring(const rust::String& str);
}

#endif //MCRESTOOL_MCRTLIB_H
