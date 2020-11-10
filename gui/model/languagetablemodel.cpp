#include <QDir>
#include <QUrl>
#include <QSharedPointer>
#include <utility>
#include "languagetablemodel.h"

using std::optional;

LanguageTableModel::LanguageTableModel(LanguageTable lt, QObject* parent) : QAbstractTableModel(parent), m_lt(std::move(lt)) {}

void LanguageTableModel::set_entry(QString language, QString key, QString value) {
    m_lt.insert(language, key, value);
    emit changed(language, key, value);
}

LanguageTableModel* LanguageTableModel::from_dir(const mcrtlib::ffi::DataSource& ds, QString path, QObject* parent) {
    return new LanguageTableModel(LanguageTable(), parent);
}

int LanguageTableModel::rowCount(const QModelIndex& parent) const {
    return m_lt.key_count();
}

int LanguageTableModel::columnCount(const QModelIndex& parent) const {
    return m_lt.language_count();
}

QVariant LanguageTableModel::data(const QModelIndex& index, int role) const {
    if (role == Qt::DisplayRole || role == Qt::EditRole) {
        QString column = get_column_name(index.column());
        QString row = get_row_name(index.row());
        QString str = m_lt.get(column.toLocal8Bit(), row.toLocal8Bit());
        if (!str.isNull()) {
            return str;
        }
    }
    return QVariant();
}

QVariant LanguageTableModel::headerData(int section, Qt::Orientation orientation, int role) const {
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

QString LanguageTableModel::get_column_name(int idx) const {
    return m_lt.get_language_at(idx);
}

QString LanguageTableModel::get_row_name(int idx) const {
    return m_lt.get_key_at(idx);
}

bool LanguageTableModel::setData(const QModelIndex& index, const QVariant& value, int role) {
    if (role == Qt::EditRole) {
        auto lang = get_column_name(index.column());
        auto key = get_row_name(index.row());
        set_entry(lang, key, value.toString());
    }
    return true;
}

Qt::ItemFlags LanguageTableModel::flags(const QModelIndex& index) const {
    return Qt::ItemIsSelectable | Qt::ItemIsEditable | Qt::ItemIsEnabled;
}

void LanguageTableModel::add_locale_key(QString locale_key) {
    emit layoutAboutToBeChanged();
    m_lt.add_key(locale_key);
    emit layoutChanged();
}

void LanguageTableModel::add_language(QString language) {
    emit layoutAboutToBeChanged();
    m_lt.add_language(language);
    emit layoutChanged();
}

LanguageTable& LanguageTableModel::data() {
    return m_lt;
}
