#include "projsrc.h"

ProjectSource::ProjectSource(DataSource& src, const QString& name, QObject* parent) : QObject(parent), name(name), src(src) {

}

bool ProjectSource::read_only() {
    return src.read_only();
}
