#ifndef MCRESTOOL_LANGTBLC_H
#define MCRESTOOL_LANGTBLC_H

#include "project.h"

#include <QObject>
#include "src/model/langtabl.h"
#include "projsrc.h"

class LanguageTableContainer : public QObject {

public:
    explicit LanguageTableContainer(ProjectSource* src, QObject* parent = nullptr);

    ~LanguageTableContainer() override;

    LanguageTable* language_table();

    bool persistent() const;

    bool read_only() const;

    void delete_file();

    void save();

signals:

    void on_deleted();

private:
    ProjectSource* src;
    LanguageTable* lt;

    bool _persistent;
    bool _deleted;

};

#endif //MCRESTOOL_LANGTBLC_H
