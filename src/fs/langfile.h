#ifndef MCRESTOOL_LANGFILE_H
#define MCRESTOOL_LANGFILE_H

#include <QMap>

QMap<QString, QString> load_from_json(const QString& path);

void save_to_json(const QString& path, const QMap<QString, QString>& localization);

#endif //MCRESTOOL_LANGFILE_H
