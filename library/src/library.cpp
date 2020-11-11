#include <mcrtlib.h>
#include <lib.rs.h>

using rust::Slice;

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

    QString to_qstring(const rust::Str& str) {
        return QString::fromStdString(std::string(str));
    }

    QString to_qstring(const rust::String& str) {
        return QString::fromStdString(std::string(str));
    }

    rust::String to_rstring(const QString& str) {
        const QByteArray& array = str.toUtf8();
        return rust::String(array, array.length());
    }

    QByteArray read_all(ffi::ResFile& file) {
        QByteArray b;
        uint8_t buf[4096];
        size_t c;
        while (c = file.read(Slice<uint8_t>((uint8_t*) &buf, 4096))) {
            b.append((char*) &buf, (int) c);
        }
        return b;
    }
}