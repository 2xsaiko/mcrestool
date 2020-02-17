#include "restree.h"

ResourceTree::ResourceTree(QObject* parent) : QAbstractItemModel(parent) {

}

QModelIndex ResourceTree::index(int row, int column, const QModelIndex& parent) const {
    return parent.model()->index(row, column);
}

QModelIndex ResourceTree::parent(const QModelIndex& child) const {
    return QModelIndex();
}

int ResourceTree::rowCount(const QModelIndex& parent) const {
    if (parent == QModelIndex())
        return 5;
    else return 0;
}

int ResourceTree::columnCount(const QModelIndex& parent) const {
    return 1;
}

QVariant ResourceTree::data(const QModelIndex& index, int role) const {
    if (role == Qt::DisplayRole) {
        return "Data!";
    }
    return QVariant();
}

void ResourceTree::add_item(const ResRoot& root) {
    items.append(root);
}

ResRoot ResRoot::demo() {
    ResRoot root;

    ResDomain& domain = root.add_domain("minecraft");
    domain.create_lang();

    return root;
}

ResDomain& ResRoot::add_domain(const QString& domain) {
    return domains[domain];
}

void ResDomain::create_lang() {
    lang = true;
}
