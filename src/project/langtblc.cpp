#include "langtblc.h"

LanguageTableContainer::LanguageTableContainer(ProjectSource* src, QObject* parent) : QObject(parent),
                                                                                      src(src), lt(new LanguageTable(this)) {
    _persistent = false;
    _deleted = false;
}

LanguageTableContainer::~LanguageTableContainer() = default;

LanguageTable* LanguageTableContainer::language_table() {
    return lt;
}

bool LanguageTableContainer::persistent() const {
    return _persistent;
}

bool LanguageTableContainer::read_only() const {
    return src->read_only();
}

void LanguageTableContainer::delete_file() {
    if (read_only()) return;

    // TODO
    _deleted = true;
}

void LanguageTableContainer::save() {
    if (read_only()) return;


}
