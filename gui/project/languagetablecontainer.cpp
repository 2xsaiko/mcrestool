#include <utility>
#include <mcrtlib.h>
#include <mcrtutil.h>
#include "languagetablecontainer.h"

using mcrtlib::ffi::DataSource;
using mcrtlib::ffi::DirEntry;
using mcrtlib::ffi::ResFile;
using mcrtlib::to_qstring;
using mcrtlib::read_all;
using mcrtlib::ffi::languagetable_new;

LanguageTableContainer::LanguageTableContainer(
    DataSource ds,
    QString path,
    QObject* parent
) : QObject(parent),
    m_ds(std::move(ds)),
    m_path(std::move(path)),
    m_lt(new LanguageTableModel(languagetable_new(), this)) {
    this->m_persistent = false;
    this->m_changed = false;
    this->m_deleted = false;

    connect(m_lt, SIGNAL(changed(const QString&, const QString&, const QString&)), this, SLOT(on_changed()));
}

LanguageTableContainer::~LanguageTableContainer() {

}

LanguageTableModel* LanguageTableContainer::language_table() {
    return this->m_lt;
}

bool LanguageTableContainer::is_persistent() const {
    return this->m_persistent;
}

bool LanguageTableContainer::is_changed() const {
    return this->m_changed;
}

bool LanguageTableContainer::is_read_only() const {
    std::string str = this->m_path.toStdString();
    return this->m_ds.read_info(rust::Str(str)).read_only;
}

const QString& LanguageTableContainer::path() const {
    return this->m_path;
}

void LanguageTableContainer::save() {
    if (is_read_only()) return;

    std::string path = this->m_path.toStdString();
    this->m_lt->data().save(this->m_ds, path);

    m_persistent = true;
    m_changed = false;
}

void LanguageTableContainer::load() {
    std::string path = this->m_path.toStdString();
    this->m_lt->data() = languagetable_load(this->m_ds, path);

    m_persistent = true;
    m_changed = false;
    emit m_lt->layoutChanged();
}

void LanguageTableContainer::on_changed() {
    m_changed = true;
    emit changed();
}
