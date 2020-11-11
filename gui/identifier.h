#ifndef MCRESTOOL_IDENTIFIER_H
#define MCRESTOOL_IDENTIFIER_H

#include <QString>

class Identifier {
public:
    explicit Identifier(const QString& spec);

    Identifier(QString domain, QString path);

    [[nodiscard]] QString domain() const;

    [[nodiscard]] QString path() const;

    [[nodiscard]] QString to_string() const;

private:
    QString m_domain;
    QString m_path;
};

#endif //MCRESTOOL_IDENTIFIER_H
