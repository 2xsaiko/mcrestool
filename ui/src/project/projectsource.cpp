#include "projectsource.h"

ProjectSource::ProjectSource(DataSourceW* src, const QString& name, QObject* parent) : QObject(parent), name(name), src(src) {

}

bool ProjectSource::read_only() {
    return src->read_only();
}

LanguageTableContainer* ProjectSource::get_language_table(const QString& domain) {
    if (!languages.contains(domain)) {
        languages.insert(domain, new LanguageTableContainer(this, domain, this));
    }
    return languages.value(domain, nullptr);
}

DataSourceW* ProjectSource::data_source() {
    return src;
}
