#include "resfile.h"
#include "datasource.h"

qint64 ResFileW::readData(char* data, qint64 maxlen) {
    uintptr_t i = resfile_read(data, maxlen, inner);
    if (MCRT_ERROR) {
        setErrorString(MCRT_ERROR_TEXT);
        return -1;
    }
    return i;
}

qint64 ResFileW::writeData(const char* data, qint64 len) {
    uintptr_t i = resfile_write(data, len, inner);
    if (MCRT_ERROR) {
        setErrorString(MCRT_ERROR_TEXT);
        return -1;
    }
    return i;
}

ResFileW::~ResFileW() {
    close();
}

ResFileW::ResFileW(DataSourceW* owner, const QString& path, QObject* parent) : QIODevice(parent), owner(owner), path(path) {}

bool ResFileW::open(QIODevice::OpenMode mode) {
    if (inner) resfile_close(inner);
    datasource_open_file(owner->ds, path.toLocal8Bit(), as_open_options(mode));
    return QIODevice::open(mode);
}

void ResFileW::close() {
    QIODevice::close();
    if (inner) {
        resfile_close(inner);
        inner = nullptr;
    }
}

OpenOptions as_open_options(QIODevice::OpenMode om) {
    OpenOptions opts = OpenOptions();
    opts.read = om & QIODevice::ReadOnly;
    opts.write = om & QIODevice::WriteOnly;
    opts.create = ~om & QIODevice::ExistingOnly;
    return opts;
}