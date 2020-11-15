#include <QDir>
#include <QUrl>
#include <QSharedPointer>
#include <utility>
#include "languagetablemodel.h"

using std::optional;
using mcrtlib::ffi::LanguageTable;
using mcrtlib::ffi::languagetable_new;
using mcrtlib::to_qstring;

LanguageTableModel::LanguageTableModel(LanguageTable lt, QObject* parent) : QAbstractTableModel(parent), m_lt(std::move(lt)) {}

void LanguageTableModel::set_entry(QString language, QString key, QString value) {
    std::string s1 = language.toStdString();
    std::string s2 = key.toStdString();
    std::string s3 = value.toStdString();
    m_lt.insert(s1, s2, s3);
    emit changed(language, key, value);
}

LanguageTableModel* LanguageTableModel::from_dir(const mcrtlib::ffi::DataSource& ds, QString path, QObject* parent) {
    return new LanguageTableModel(languagetable_new(), parent);
}

int LanguageTableModel::rowCount(const QModelIndex& parent) const {
    return m_lt.key_count();
}

int LanguageTableModel::columnCount(const QModelIndex& parent) const {
    return m_lt.language_count();
}

QVariant LanguageTableModel::data(const QModelIndex& index, int role) const {
    if (role == Qt::DisplayRole || role == Qt::EditRole) {
        std::string column = get_column_name(index.column()).toStdString();
        std::string row = get_row_name(index.row()).toStdString();
        try {
            rust::String str = m_lt.get(column, row);
            return to_qstring(str);
        } catch (const rust::Str& e) {

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
    return to_qstring(m_lt.get_language_at(idx));
}

QString LanguageTableModel::get_row_name(int idx) const {
    return to_qstring(m_lt.get_key_at(idx));
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
    std::string s = locale_key.toStdString();
    m_lt.add_key(s);
    emit layoutChanged();
}

void LanguageTableModel::add_language(QString language) {
    emit layoutAboutToBeChanged();
    std::string s = language.toStdString();
    m_lt.add_language(s);
    emit layoutChanged();
}

LanguageTable& LanguageTableModel::data() {
    return m_lt;
}
