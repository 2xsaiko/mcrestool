#ifndef MCRESTOOL_RESTREE_H
#define MCRESTOOL_RESTREE_H

#include <QtCore/QString>
#include <QtCore/QAbstractItemModel>

class ResDomain {
    friend class ResourceTree;

public:
    void create_lang();

private:
    bool lang;
};

class ResRoot {
    friend class ResourceTree;

public:
    static ResRoot demo();

    ResDomain& add_domain(const QString& domain);

private:
    QMap<QString, ResDomain> domains;
};

class ResourceTree : public QAbstractItemModel {
Q_OBJECT

public:
    explicit ResourceTree(QObject* parent = nullptr);

    void add_item(const ResRoot& root);

private:
    QList<ResRoot> items;

    QModelIndex index(int row, int column, const QModelIndex& parent) const override;

    QModelIndex parent(const QModelIndex& child) const override;

    int rowCount(const QModelIndex& parent) const override;

    int columnCount(const QModelIndex& parent) const override;

    QVariant data(const QModelIndex& index, int role) const override;


};

#endif //MCRESTOOL_RESTREE_H
