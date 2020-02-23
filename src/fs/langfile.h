#ifndef MCRESTOOL_LANGFILE_H
#define MCRESTOOL_LANGFILE_H

#include <QMap>
#include "datasource.h"

namespace langfile {
    QMap<QString, QString> load_from_json(DataSource& source, const QString& path);

    void save_to_json(DataSource& source, const QString& path, const QMap<QString, QString>& localization);
}

#endif //MCRESTOOL_LANGFILE_H
