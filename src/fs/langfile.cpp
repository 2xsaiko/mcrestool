#include <QJsonDocument>
#include <QFile>
#include <QJsonObject>
#include "src/util.h"
#include "langfile.h"

namespace langfile {
    QMap<QString, QString> load_from_json(const QString& path) {
        QMap<QString, QString> map;
        QFile file(path);
        if (!file.open(QIODevice::ReadOnly | QIODevice::Text)) {
            unimplemented(); // TODO: implement error handling
        }
        QString content = file.readAll();
        file.close();
        QJsonDocument doc = QJsonDocument::fromJson(content.toUtf8());
        if (!doc.isObject()) {
            unimplemented(); // TODO: implement error handling
        }
        QJsonObject obj = doc.object();
        auto iterator = obj.begin();
        while (iterator != obj.end()) {
            map.insert(iterator.key(), iterator->toString());
            iterator += 1;
        }
        return map;
    }

    void save_to_json(const QString& path, const QMap<QString, QString>& localization) {
        QFile file = QFile(path);
        if (!file.open(QIODevice::WriteOnly | QIODevice::Truncate | QIODevice::Text)) {
            unimplemented(); // TODO: implement error handling
        }
        QJsonObject obj;
        auto iterator = localization.begin();
        while (iterator != localization.end()) {
            obj.insert(iterator.key(), iterator.value());
        }
        QJsonDocument doc = QJsonDocument(obj);
        QString str = doc.toJson();
        file.write(str.toUtf8());
        file.close();
    }
}
