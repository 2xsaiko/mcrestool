#ifndef MCRESTOOL_RUSTITEMMODEL_H
#define MCRESTOOL_RUSTITEMMODEL_H

#include <QAbstractItemModel>
#include <mcrtlib.h>

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

protected:
    virtual T get_data(quintptr ptr) = 0;

};

template<typename T>
RustItemModel<T>::RustItemModel(QObject* parent) : RustItemModelBase(parent) {
}

#endif //MCRESTOOL_RUSTITEMMODEL_H
