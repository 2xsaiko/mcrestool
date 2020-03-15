#include <QDir>
#include <QUrl>
#include <QSet>
#include <QSharedPointer>
#include "languagetablemodel.h"

using std::optional;

LanguageTableModel::LanguageTableModel(LanguageTable* lt, QObject* parent) : QAbstractTableModel(parent), lt(lt) {}

void LanguageTableModel::set_entry(QString language, QString key, QString value) {
    languagetable_insert(lt, language.toLocal8Bit(), key.toLocal8Bit(), value.toLocal8Bit());
    emit changed(language, key, value);
}

LanguageTableModel* LanguageTableModel::from_dir(QObject* parent, QString path) {
    QDir dir(path);
    return new LanguageTableModel(languagetable_create(), parent);
}

int LanguageTableModel::rowCount(const QModelIndex& parent) const {
    return languagetable_row_count(lt);
}

int LanguageTableModel::columnCount(const QModelIndex& parent) const {
    return languagetable_col_count(lt);
}

QVariant LanguageTableModel::data(const QModelIndex& index, int role) const {
    if (role == Qt::DisplayRole || role == Qt::EditRole) {
        QString column = get_column_name(index.column());
        QString row = get_row_name(index.row());
        const char* str = languagetable_get(lt, column.toLocal8Bit(), row.toLocal8Bit());
        QString content = QString(str);
        languagetable_content_delete(str);
        if (!content.isNull()) {
            return content;
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
    return languagetable_get_col_name(lt, idx);
}

QString LanguageTableModel::get_row_name(int idx) const {
    return languagetable_get_row_name(lt, idx);
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
    languagetable_add_localization_key(lt, locale_key.toLocal8Bit());
    emit layoutChanged();
}

void LanguageTableModel::add_language(QString language) {
    emit layoutAboutToBeChanged();
    languagetable_add_language(lt, language.toLocal8Bit());
    emit layoutChanged();
}

LanguageTable* LanguageTableModel::data() {
    return lt;
}
