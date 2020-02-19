#ifndef MCRESTOOL_IDENTIFR_H
#define MCRESTOOL_IDENTIFR_H

#include <QString>

class Identifier {
public:
    explicit Identifier(QString spec);

    Identifier(QString domain, QString path);

    [[nodiscard]] QString domain() const;

    [[nodiscard]] QString path() const;

    [[nodiscard]] QString to_string() const;

private:
    QString _domain;
    QString _path;
};

#endif //MCRESTOOL_IDENTIFR_H
