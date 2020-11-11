#include "identifier.h"

#include <utility>

Identifier::Identifier(const QString& spec) {
    int i = spec.indexOf(':');
    if (i == -1) {
        m_domain = "minecraft";
        m_path = spec;
    } else {
        m_domain = spec.left(i);
        m_path = spec.mid(i + 1);
    }
}

Identifier::Identifier(QString domain, QString path) {
    m_domain = std::move(domain);
    m_path = std::move(path);
}

QString Identifier::domain() const {
    return m_domain;
}

QString Identifier::path() const {
    return m_path;
}

QString Identifier::to_string() const {
    return m_domain + ":" + m_path;
}
