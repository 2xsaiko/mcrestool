#ifndef MCRESTOOL_LANGUAGETABLE_H
#define MCRESTOOL_LANGUAGETABLE_H

#include <QString>

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

};

#endif //MCRESTOOL_LANGUAGETABLE_H
