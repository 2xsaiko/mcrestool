#ifndef MCRESTOOL_PROJSRC_H
#define MCRESTOOL_PROJSRC_H

#include "src/fs/datasrc.h"
#include "datapack.h"
#include "respack.h"
#include <QObject>
#include <QMap>

class ProjectSource : public QObject {
Q_OBJECT

public:
    explicit ProjectSource(DataSource* src, const QString& name, QObject* parent = nullptr);


    bool read_only();

private:
    QString name;

    QMap<QString, DataPack> data_packs;
    QMap<QString, ResourcePack> resource_packs;

    DataSource* src;

};

#endif //MCRESTOOL_PROJSRC_H
