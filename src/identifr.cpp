#include "identifr.h"

Identifier::Identifier(QString spec) {
    int i = spec.indexOf(':');
    if (i == -1) {
        _domain = "minecraft";
        _path = spec;
    } else {
        _domain = spec.left(i);
        _path = spec.mid(i + 1);
    }
}

Identifier::Identifier(QString domain, QString path) {
    _domain = domain;
    _path = path;
}

QString Identifier::domain() const {
    return _domain;
}

QString Identifier::path() const {
    return _path;
}

QString Identifier::to_string() const {
    return _domain + ":" + _path;
}
