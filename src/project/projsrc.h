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

    LanguageTableContainer* get_language_table(const QString& domain);

    bool read_only();

    bool changed();

    DataSource& data_source();

private:
    QString name;

    DataSource& src;

    QMap<QString, LanguageTableContainer*> languages;

};

#endif //MCRESTOOL_PROJSRC_H
