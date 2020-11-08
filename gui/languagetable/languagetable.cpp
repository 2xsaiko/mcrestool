#include "languagetable.h"

void LanguageTable::insert(QString language, QString key, QString value) {
    this->add_language(language);
    this->add_key(key);
    this->table[language][key] = value;
}

void LanguageTable::add_key(QString key) {
    if (!this->keys.contains(key)) {
        this->keys += key;
    }
}

void LanguageTable::add_language(QString language) {
    if (!this->languages.contains(language)) {
        this->languages += language;
    }
}

int LanguageTable::key_count() const {
    return this->keys.size();
}

int LanguageTable::language_count() const {
    return this->languages.size();
}

QString LanguageTable::get(const QString& language, const QString& key) const {
    return this->table[language][key];
}

QString LanguageTable::get_language_at(int index) const {
    return this->languages[index];
}

QString LanguageTable::get_key_at(int index) const {
    return this->keys[index];
}

QStringList LanguageTable::get_keys_for(const QString& language) const {
    return this->table[language].keys();
}

QMap<QString, QString> LanguageTable::get_entries_for(const QString& language) const {
    return this->table[language];
}

bool LanguageTable::contains_language(const QString& language) const {
    return this->table.contains(language);
}

void LanguageTable::clear() {
    this->table.clear();
    this->keys.clear();
    this->languages.clear();
}
