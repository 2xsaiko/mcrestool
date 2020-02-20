#ifndef MCRESTOOL_PROJSRC_H
#define MCRESTOOL_PROJSRC_H

#include "project.h"

#include "src/fs/datasrc.h"
#include "langtblc.h"
#include <QObject>
#include <QMap>

// use QFileSystemWatcher?

class ProjectSource : public QObject {
Q_OBJECT

public:
    explicit ProjectSource(DataSource& src, const QString& name, QObject* parent = nullptr);

    bool read_only();

private:
    QString name;

    DataSource& src;

    QMap<QString, LanguageTableContainer*> languages;

};

#endif //MCRESTOOL_PROJSRC_H
