#include "languagetablecontainer.h"

LanguageTableContainer::LanguageTableContainer(
    ProjectSource* src,
    const QString& domain,
    QObject* parent
) : QObject(parent),
    src(src),
    lt(new LanguageTableModel(languagetable_create(), this)),
    domain(domain) {
    _persistent = false;
    _changed = false;
    _deleted = false;

    connect(lt, SIGNAL(changed(
                           const QString&, const QString&, const QString&)), this, SLOT(on_changed()));
}

LanguageTableContainer::~LanguageTableContainer() {
    languagetable_delete(lt->data());
}

LanguageTableModel* LanguageTableContainer::language_table() {
    return lt;
}

bool LanguageTableContainer::persistent() const {
    return _persistent;
}

bool LanguageTableContainer::changed() const {
    return _changed;
}

bool LanguageTableContainer::read_only() const {
    return src->read_only();
}

void LanguageTableContainer::delete_file() {
    if (read_only()) return;

    if (persistent()) {
        QStringList files = src->data_source()->list_dir("/assets/" + domain + "/lang/");
        for (const auto& str: files) {
            src->data_source()->delete_file(str);
        }
    }
    _deleted = true;
    _persistent = false;
}

void LanguageTableContainer::save() {
    if (read_only()) return;

    languagetable_write_to(lt->data(), src->data_source()->inner(), ("/assets/" + domain + "/lang/").toLocal8Bit());

    _persistent = true;
    _changed = false;
}

void LanguageTableContainer::load() {
    languagetable_load_into(lt->data(), src->data_source()->inner(),  ("/assets/" + domain + "/lang/").toLocal8Bit());

    _persistent = true;
    _changed = false;
    emit lt->layoutChanged();
}

void LanguageTableContainer::on_changed() {
    _changed = true;
    emit changed();
}
