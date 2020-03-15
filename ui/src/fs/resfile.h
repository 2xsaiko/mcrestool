#ifndef MCRESTOOL_RESFILE_H
#define MCRESTOOL_RESFILE_H

#include <QtCore/QIODevice>
#include "mcrestool_logic.h"

class DataSourceW;

class ResFileW : public QIODevice {
Q_OBJECT

    friend class DataSourceW;

public:
    ~ResFileW() override;

protected:
    explicit ResFileW(DataSourceW* owner, const QString& path, QObject* parent = nullptr);

    qint64 readData(char* data, qint64 maxlen) override;

    qint64 writeData(const char* data, qint64 len) override;

public:
    bool open(OpenMode mode) override;

    void close() override;

private:
    DataSourceW* owner;
    QString path;

    ResFile* inner;

};

static OpenOptions as_open_options(QIODevice::OpenMode om);

#endif //MCRESTOOL_RESFILE_H
