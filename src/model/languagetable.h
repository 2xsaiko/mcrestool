#ifndef MCRESTOOL_LANGUAGETABLE_H
#define MCRESTOOL_LANGUAGETABLE_H

#include <QObject>
#include <QMap>
#include <QAbstractTableModel>
#include "src/table.h"

class LanguageTable : public QAbstractTableModel {
Q_OBJECT

public:
    explicit LanguageTable(QObject* parent = nullptr);

    void set_entry(QString language, QString key, QString value);

    static LanguageTable* from_dir(QObject* parent, QString path);

    [[nodiscard]] int rowCount(const QModelIndex& parent) const override;

    [[nodiscard]] int columnCount(const QModelIndex& parent) const override;

    [[nodiscard]] QVariant data(const QModelIndex& index, int role) const override;

    [[nodiscard]] QVariant headerData(int section, Qt::Orientation orientation, int role) const override;

    bool setData(const QModelIndex& index, const QVariant& value, int role) override;

    [[nodiscard]] Qt::ItemFlags flags(const QModelIndex& index) const override;

    void add_locale_key(QString locale_key);

    void add_language(QString language);

    [[nodiscard]] const Table<QString, QString, QString>& data() const;

signals:

    void changed(const QString& language, const QString& key, const QString& value);

private:
    Table<QString, QString, QString> table;
    QList<QString> languages;
    QList<QString> locale_keys;

    [[nodiscard]] QString get_column_name(int idx) const;

    [[nodiscard]] QString get_row_name(int idx) const;

};

#endif //MCRESTOOL_LANGUAGETABLE_H
