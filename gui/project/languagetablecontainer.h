#ifndef MCRESTOOL_LANGUAGETABLECONTAINER_H
#define MCRESTOOL_LANGUAGETABLECONTAINER_H

#include <QObject>
#include <mcrtlib.h>
#include <languagetablemodel.h>

class LanguageTableContainer : public QObject {
Q_OBJECT

public:
    explicit LanguageTableContainer(mcrtlib::ffi::DataSource ds, QString path, QObject* parent = nullptr);

    ~LanguageTableContainer() override;

    LanguageTableModel* language_table();

    [[nodiscard]] bool is_persistent() const;

    [[nodiscard]] bool is_changed() const;

    [[nodiscard]] bool is_read_only() const;

    [[nodiscard]] const QString& path() const;

    void save();

    void load();

public slots:

    void on_changed();

signals:

    void deleted();

    void changed();

private:
    mcrtlib::ffi::DataSource m_ds;
    QString m_path;
    LanguageTableModel* m_lt;

    bool m_persistent;
    bool m_changed;
    bool m_deleted;
};

#endif //MCRESTOOL_LANGUAGETABLECONTAINER_H
