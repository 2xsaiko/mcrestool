#ifndef MCRESTOOL_FSTREEMODEL_H
#define MCRESTOOL_FSTREEMODEL_H

#include "rustitemmodel.h"

class FsTreeModel : public RustItemModel<mcrtlib::ffi::FsTreeEntry> {
Q_OBJECT

public:
    explicit FsTreeModel(mcrtlib::ffi::Workspace& ws, QObject* parent = nullptr);

    ~FsTreeModel() override;

protected:
    [[nodiscard]] mcrtlib::ffi::FsTreeEntry get_data(quintptr ptr) const override;

    [[nodiscard]] QVariant get_display(const mcrtlib::ffi::FsTreeEntry& data, int role) const override;

    [[nodiscard]] size_t index_of(const mcrtlib::ffi::FsTreeEntry& data) const override;

    [[nodiscard]] std::optional<mcrtlib::ffi::FsTreeEntry> get_parent(const mcrtlib::ffi::FsTreeEntry& data) const override;

    [[nodiscard]] size_t children_count(optional_ref<const mcrtlib::ffi::FsTreeEntry> data) const override;

    [[nodiscard]] mcrtlib::ffi::FsTreeEntry index(optional_ref<const mcrtlib::ffi::FsTreeEntry> data, size_t row) const override;

private:
    mcrtlib::ffi::Workspace& ws;

};

#endif //MCRESTOOL_FSTREEMODEL_H
