#ifndef MCRESTOOL_LANGUAGETABLE_H
#define MCRESTOOL_LANGUAGETABLE_H

#include <QString>
#include <QMap>

class LanguageTable {

public:
    void insert(QString language, QString key, QString value);

    void add_key(QString key);

    void add_language(QString language);

    int key_count() const;

    int language_count() const;

    QString get(const QString& language, const QString& key) const;

    QString get_language_at(int index) const;

    QString get_key_at(int index) const;

    QStringList get_keys_for(const QString& language) const;

    QMap<QString, QString> get_entries_for(const QString& language) const;

    bool contains_language(const QString& language) const;

    void clear();

private:
    QStringList languages;
    QStringList keys;
    QMap<QString, QMap<QString, QString>> table;

};

#endif //MCRESTOOL_LANGUAGETABLE_H
