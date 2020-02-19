#include "restree.h"

ResourceTree::ResourceTree(QObject* parent) :
    QAbstractItemModel(parent) {
    root_item = new TreeItem(QString());

    {
        auto mcjar = new TreeItem("minecraft.jar", root_item);
        auto mcnamespace = new TreeItem("minecraft", mcjar);

        auto mcassets = new TreeItem("Assets", mcnamespace);

        mcassets->append_child(new TreeItem("Localization", mcassets));
        mcnamespace->append_child(mcassets);

        auto mcdata = new TreeItem("Data", mcnamespace);

        auto recipes = new TreeItem("Recipes", mcdata);
        recipes->append_child(new TreeItem("recipe_a", recipes));
        recipes->append_child(new TreeItem("recipe_b", recipes));
        recipes->append_child(new TreeItem("recipe_c", recipes));
        recipes->append_child(new TreeItem("recipe_d", recipes));
        mcdata->append_child(recipes);
        mcnamespace->append_child(mcdata);

        mcjar->append_child(mcnamespace);
        root_item->append_child(mcjar);
    }

    {
        auto mcjar = new TreeItem("resources", root_item);
        auto mcnamespace = new TreeItem("rswires", mcjar);

        auto mcassets = new TreeItem("Assets", mcnamespace);

        mcassets->append_child(new TreeItem("Localization", mcassets));
        mcnamespace->append_child(mcassets);

        auto mcdata = new TreeItem("Data", mcnamespace);

        auto recipes = new TreeItem("Recipes", mcdata);
        recipes->append_child(new TreeItem("bundled_cable", recipes));
        recipes->append_child(new TreeItem("red_alloy_wire", recipes));
        recipes->append_child(new TreeItem("white_insulated_wire", recipes));
        mcdata->append_child(recipes);
        mcnamespace->append_child(mcdata);

        mcjar->append_child(mcnamespace);
        root_item->append_child(mcjar);
    }

}

ResourceTree::~ResourceTree() {
    delete root_item;
}

QModelIndex ResourceTree::index(int row, int column, const QModelIndex& parent) const {
    if (!hasIndex(row, column, parent)) return QModelIndex();

    TreeItem* parent_item;

    if (!parent.isValid()) {
        parent_item = root_item;
    } else {
        parent_item = static_cast<TreeItem*>(parent.internalPointer());
    }

    TreeItem* child_item = parent_item->child(row);

    if (!child_item) return QModelIndex();

    return createIndex(row, column, child_item);
}

QModelIndex ResourceTree::parent(const QModelIndex& child) const {
    if (!child.isValid()) return QModelIndex();

    auto* item = static_cast<TreeItem*>(child.internalPointer());
    TreeItem* parent = item->parent_item();

    if (parent == root_item) return QModelIndex();

    return createIndex(parent->row(), 0, parent);
}

QVariant ResourceTree::data(const QModelIndex& index, int role) const {
    if (!index.isValid()) return QVariant();
    if (role != Qt::DisplayRole) return QVariant();

    auto* item = static_cast<TreeItem*>(index.internalPointer());

    return item->text();
}

Qt::ItemFlags ResourceTree::flags(const QModelIndex& index) const {
    if (!index.isValid())return Qt::NoItemFlags;

    return QAbstractItemModel::flags(index);
}

QVariant ResourceTree::headerData(int, Qt::Orientation, int) const {
    return QVariant();
}

int ResourceTree::rowCount(const QModelIndex& parent) const {
    if (parent.column() > 0) return 0;

    if (!parent.isValid()) return root_item->child_count();

    auto* parent_item = static_cast<TreeItem*>(parent.internalPointer());
    return parent_item->child_count();
}

int ResourceTree::columnCount(const QModelIndex& parent) const {
    return 1;
}
