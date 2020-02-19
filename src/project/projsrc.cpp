#include "projsrc.h"

ProjectSource::ProjectSource(DataSource* src, const QString& name, QObject* parent) {

}

bool ProjectSource::read_only() {
    return src->read_only();
}
