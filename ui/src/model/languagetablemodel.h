#ifndef MCRESTOOL_LANGUAGETABLEMODEL_H
#define MCRESTOOL_LANGUAGETABLEMODEL_H

#include <QObject>
#include <QMap>
#include <QAbstractTableModel>
#include "mcrestool_logic.h"

class LanguageTableModel : public QAbstractTableModel {
Q_OBJECT

public:
    explicit LanguageTableModel(LanguageTable* lt, QObject* parent = nullptr);

    void set_entry(QString language, QString key, QString value);

    static LanguageTableModel* from_dir(QObject* parent, QString path);

    [[nodiscard]] int rowCount(const QModelIndex& parent) const override;

    [[nodiscard]] int columnCount(const QModelIndex& parent) const override;

    [[nodiscard]] QVariant data(const QModelIndex& index, int role) const override;

    [[nodiscard]] QVariant headerData(int section, Qt::Orientation orientation, int role) const override;

    bool setData(const QModelIndex& index, const QVariant& value, int role) override;

    [[nodiscard]] Qt::ItemFlags flags(const QModelIndex& index) const override;

    void add_locale_key(QString locale_key);

    void add_language(QString language);

    [[nodiscard]] LanguageTable* data() ;

signals:

    void changed(const QString& language, const QString& key, const QString& value);

private:
    LanguageTable* lt;

    [[nodiscard]] QString get_column_name(int idx) const;

    [[nodiscard]] QString get_row_name(int idx) const;

};

#endif //MCRESTOOL_LANGUAGETABLEMODEL_H
