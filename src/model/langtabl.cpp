#include <QDir>
#include <QUrl>
#include <QSet>
#include <QSharedPointer>
#include "langtabl.h"

using std::optional;

void LanguageTable::set_entry(QString language, QString key, QString value) {
    add_language(language);
    add_locale_key(key);
    table.insert(language, key, value);
    emit changed(language, key, value);
}

LanguageTable* LanguageTable::from_dir(QObject* parent, QString path) {
    QDir dir(path);
    return new LanguageTable(parent);
}

LanguageTable::LanguageTable(QObject* parent) : QAbstractTableModel(parent) {}

int LanguageTable::rowCount(const QModelIndex& parent) const {
    return locale_keys.size();
}

int LanguageTable::columnCount(const QModelIndex& parent) const {
    return languages.size();
}

QVariant LanguageTable::data(const QModelIndex& index, int role) const {
    if (role == Qt::DisplayRole || role == Qt::EditRole) {
        QString column = get_column_name(index.column());
        QString row = get_row_name(index.row());
        const optional<QString> text = table.get(column, row);
        if (text) {
            return QString(text.value());
        }
    }
    return QVariant();
}

QVariant LanguageTable::headerData(int section, Qt::Orientation orientation, int role) const {
    if (role == Qt::DisplayRole) {
        switch (orientation) {
            case Qt::Horizontal:
                return get_column_name(section);
            case Qt::Vertical:
                return get_row_name(section);
        }
    }
    return QVariant();
}

QString LanguageTable::get_row_name(int idx) const {
    return locale_keys[idx];
}

QString LanguageTable::get_column_name(int idx) const {
    return languages[idx];
}

bool LanguageTable::setData(const QModelIndex& index, const QVariant& value, int role) {
    if (role == Qt::EditRole) {
        auto lang = get_column_name(index.column());
        auto key = get_row_name(index.row());
        set_entry(lang, key, value.toString());
    }
    return true;
}

Qt::ItemFlags LanguageTable::flags(const QModelIndex& index) const {
    return Qt::ItemIsSelectable | Qt::ItemIsEditable | Qt::ItemIsEnabled;
}

void LanguageTable::add_locale_key(QString locale_key) {
    if (!locale_keys.contains(locale_key)) {
        emit layoutAboutToBeChanged();
        locale_keys.append(locale_key);
        emit layoutChanged();
    }
}

void LanguageTable::add_language(QString language) {
    if (!languages.contains(language)) {
        emit layoutAboutToBeChanged();
        languages.append(language);
        emit layoutChanged();
    }
}
