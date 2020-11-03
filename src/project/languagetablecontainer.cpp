#include <direntry.h>
#include "languagetablecontainer.h"

#include <QJsonDocument>
#include <QJsonObject>
#include <QDebug>

LanguageTableContainer::LanguageTableContainer(
    FsRef fs_ref,
    QObject* parent
) : QObject(parent),
    fs_ref(fs_ref),
    lt(new LanguageTableModel(LanguageTable(), this)) {
    this->_persistent = false;
    this->_changed = false;
    this->_deleted = false;

    connect(lt, SIGNAL(changed(const QString&, const QString&, const QString&)), this, SLOT(on_changed()));
}

LanguageTableContainer::~LanguageTableContainer() {

}

LanguageTableModel* LanguageTableContainer::language_table() {
    return this->lt;
}

bool LanguageTableContainer::persistent() const {
    return this->_persistent;
}

bool LanguageTableContainer::changed() const {
    return this->_changed;
}

bool LanguageTableContainer::read_only() const {
    return this->fs_ref.read_only();
}

void LanguageTableContainer::save() {
    if (read_only()) return;

    for (auto entry: this->fs_ref.read_dir()) {
        qDebug() << entry.file_name;
        if (entry.file_name.endsWith(".json") && entry.is_file) {
            QString lang = entry.file_name.left(entry.file_name.length() - 5);
            if (!this->lt->data().contains_language(lang)) {
                entry.real_path.remove(false);
            }
        }
    }

    for (int i = 0; i < this->lt->data().language_count(); i++) {
        QString lang = this->lt->data().get_language_at(i);
        QJsonObject obj;
        QMap<QString, QString> map = this->lt->data().get_entries_for(lang);
        if (!map.isEmpty()) {
            for (auto key: map.keys()) {
                QString value = map[key];
                if (!value.isEmpty()) {
                    obj.insert(key, value);
                }
            }
        }
        QJsonDocument d;
        d.setObject(obj);
        FsRef lang_file = this->fs_ref.join(lang + ".json");

        QSharedPointer<QIODevice> dev = lang_file.open();
        dev->open(QIODevice::ReadWrite | QIODevice::Truncate | QIODevice::Text);
        dev->write(d.toJson(QJsonDocument::Compact));
        dev->close();
    }

    _persistent = true;
    _changed = false;
}

void LanguageTableContainer::load() {
    this->lt->data().clear();

    QList<DirEntry> list = this->fs_ref.read_dir();

    // move en_us to the beginning
    for (int i = 0; i < list.size(); i++) {
        DirEntry entry = list[i];
        if (entry.file_name.endsWith(".json") && entry.is_file) {
            QString lang = entry.file_name.left(entry.file_name.length() - 5);
            if (lang == "en_us") {
                list.removeAt(i);
                list.insert(0, entry);
                break;
            }
        }
    }

    for (auto entry: list) {
        if (entry.file_name.endsWith(".json") && entry.is_file) {
            QString lang = entry.file_name.left(entry.file_name.length() - 5);
            this->lt->data().add_language(lang);
            QSharedPointer<QIODevice> dev = entry.real_path.open();
            dev->open(QIODevice::ReadOnly | QIODevice::Text);
            QJsonParseError err;
            auto doc = QJsonDocument::fromJson(dev->readAll(), &err);

            // TODO actually show errors
            if (err.error != QJsonParseError::NoError) continue;
            if (!doc.isObject()) continue;

            QJsonObject object = doc.object();
            for (QString key: object.keys()) {
                QJsonValueRef value = object[key];
                if (!value.isString()) continue;
                this->lt->data().insert(lang, key, value.toString());
            }
        }
    }

    _persistent = true;
    _changed = false;
    emit lt->layoutChanged();
}

void LanguageTableContainer::on_changed() {
    _changed = true;
    emit changed();
}
