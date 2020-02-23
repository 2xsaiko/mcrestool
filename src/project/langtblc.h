#ifndef MCRESTOOL_LANGTBLC_H
#define MCRESTOOL_LANGTBLC_H

#include "project.h"

#include <QObject>
#include "src/model/langtabl.h"
#include "projsrc.h"

class LanguageTableContainer : public QObject {
Q_OBJECT

public:
    explicit LanguageTableContainer(ProjectSource* src, const QString& domain, QObject* parent = nullptr);

    ~LanguageTableContainer() override;

    LanguageTable* language_table();

    bool persistent() const;

    bool changed() const;

    bool read_only() const;

    void delete_file();

    void save();

public slots:

    void on_changed();

signals:

    void deleted();

    void changed();

private:
    ProjectSource* src;
    LanguageTable* lt;
    QString domain;

    bool _persistent;
    bool _changed;
    bool _deleted;

};

#endif //MCRESTOOL_LANGTBLC_H
