#ifndef MCRESTOOL_RUSTITEMMODEL_H
#define MCRESTOOL_RUSTITEMMODEL_H

#include <QAbstractItemModel>
#include <mcrtlib.h>
#include <mcrtutil.h>

class RustItemModelBase : public QAbstractItemModel, public mcrtlib::ffi::TreeChangeSubscriber {

public:
    explicit RustItemModelBase(QObject* parent = nullptr);

    void pre_insert(const rust::Vec<size_t>& path, size_t start, size_t end) override;

    void post_insert(const rust::Vec<size_t>& path) override;

    void pre_remove(const rust::Vec<size_t>& path, size_t start, size_t end) override;

    void post_remove(const rust::Vec<size_t>& path) override;

protected:
    QModelIndex find_path(const rust::Vec<size_t>& path);

};

template<typename T>
class RustItemModel : public RustItemModelBase {

public:
    explicit RustItemModel(QObject* parent = nullptr);

    QModelIndex index(int row, int column, const QModelIndex& parent) const override;

    QModelIndex parent(const QModelIndex& child) const override;

    int rowCount(const QModelIndex& parent) const override;

    int columnCount(const QModelIndex& parent) const override;

    QVariant data(const QModelIndex& index, int role) const override;

    QVariant headerData(int section, Qt::Orientation orientation, int role) const override;

protected:
    virtual T get_data(quintptr ptr) const = 0;

    std::optional<T> get_data(const QModelIndex& index) const;

    virtual QVariant get_display(const T& data, int role) const = 0;

    QModelIndex to_index(const T& data) const;

    virtual size_t index_of(const T& data) const = 0;

    virtual std::optional<T> get_parent(const T& data) const = 0;

    virtual size_t children_count(optional_ref<const T> data) const = 0;

    virtual T index(optional_ref<const T>, size_t row) const = 0;

};

template<typename T>
RustItemModel<T>::RustItemModel(QObject* parent) : RustItemModelBase(parent) {
}

template<typename T>
std::optional<T> RustItemModel<T>::get_data(const QModelIndex& index) const {
    if (!index.isValid()) {
        return std::optional<T>();
    } else {
        return this->get_data(index.internalId());
    }
}

template<typename T>
QModelIndex RustItemModel<T>::index(int row, int column, const QModelIndex& parent) const {
    if (!hasIndex(row, column, parent)) return QModelIndex();

    std::optional<T> node = this->get_data(parent);

    return this->to_index(this->index(node, row));
}

template<typename T>
QModelIndex RustItemModel<T>::parent(const QModelIndex& child) const {
    std::optional<T> parent = this->get_parent(*this->get_data(child));
    if (parent) {
        return this->to_index(*parent);
    } else {
        return QModelIndex();
    }
}

template<typename T>
QModelIndex RustItemModel<T>::to_index(const T& data) const {
    size_t index = this->index_of(data);
    return this->createIndex(index, 0, data.to_ptr());
}

template<typename T>
int RustItemModel<T>::rowCount(const QModelIndex& parent) const {
    if (parent.isValid()) {
        return this->children_count(this->get_data(parent));
    } else {
        return this->children_count(std::optional<T>());
    }
}

template<typename T>
int RustItemModel<T>::columnCount(const QModelIndex& parent) const {
    return 1;
}

template<typename T>
QVariant RustItemModel<T>::data(const QModelIndex& index, int role) const {
    return this->get_display(*this->get_data(index), role);
}

template<typename T>
QVariant RustItemModel<T>::headerData(int section, Qt::Orientation orientation, int role) const {
    return QVariant();
}

#endif //MCRESTOOL_RUSTITEMMODEL_H
