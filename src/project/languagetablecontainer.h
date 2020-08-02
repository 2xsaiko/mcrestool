#ifndef MCRESTOOL_LANGUAGETABLECONTAINER_H
#define MCRESTOOL_LANGUAGETABLECONTAINER_H

#include "src/workspace/fsref.h"
#include "src/model/languagetablemodel.h"

#include <QObject>

class LanguageTableContainer : public QObject {
Q_OBJECT

public:
    explicit LanguageTableContainer(FsRef fs_ref, QObject* parent = nullptr);

    ~LanguageTableContainer();

    LanguageTableModel* language_table();

    bool persistent() const;

    bool changed() const;

    bool read_only() const;

    void save();

    void load();

public slots:

    void on_changed();

signals:

    void deleted();

    void changed();

private:
    FsRef fs_ref;
    LanguageTableModel* lt;

    bool _persistent;
    bool _changed;
    bool _deleted;

};

#endif //MCRESTOOL_LANGUAGETABLECONTAINER_H
