#include <QJsonDocument>
#include <QJsonObject>
#include <QDebug>
#include <QList>
#include <utility>
#include <mcrtlib.h>
#include <path.h>
#include <mcrtutil.h>
#include "languagetablecontainer.h"

using mcrtlib::ffi::DataSource;
using mcrtlib::ffi::DirEntry;

LanguageTableContainer::LanguageTableContainer(
    const DataSource& ds,
    QString path,
    QObject* parent
) : QObject(parent),
    m_ds(ds),
    m_path(std::move(path)),
    m_lt(new LanguageTableModel(LanguageTable(), this)) {
    this->m_persistent = false;
    this->m_changed = false;
    this->m_deleted = false;

    connect(m_lt, SIGNAL(changed(const QString&, const QString&, const QString&)), this, SLOT(on_changed()));
}

LanguageTableContainer::~LanguageTableContainer() {

}

LanguageTableModel* LanguageTableContainer::language_table() {
    return this->m_lt;
}

bool LanguageTableContainer::persistent() const {
    return this->m_persistent;
}

bool LanguageTableContainer::changed() const {
    return this->m_changed;
}

bool LanguageTableContainer::read_only() const {
    return this->m_ds.read_info(TO_RUST_STR(this->m_path)).read_only;
}

void LanguageTableContainer::save() {
    if (read_only()) return;

    for (const auto& entry: this->m_ds.list_dir(TO_RUST_STR(this->m_path))) {
        const QString& file_name = TO_QSTR(entry.file_name);
        qDebug() << file_name;
        if (Path(file_name).extension() == "json" && entry.info.is_file) {
            QString lang = Path(file_name).file_stem();
            if (!this->m_lt->data().contains_language(lang)) {
                this->m_ds.delete_file(TO_RUST_STR(this->m_path + "/" + file_name));
            }
        }
    }

    for (int i = 0; i < this->m_lt->data().language_count(); i++) {
        QString lang = this->m_lt->data().get_language_at(i);
        QJsonObject obj;
        QMap<QString, QString> map = this->m_lt->data().get_entries_for(lang);
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

        QString path = this->m_path + "/" + lang + ".json";
        mcrtlib::ffi::ResFile file = this->m_ds.open(TO_RUST_STR(path), "wct");
        const QByteArray& array = d.toJson(QJsonDocument::Compact);
        file.write(rust::Slice<uint8_t>((const uint8_t*) array.constData(), array.length()));
    }

    m_persistent = true;
    m_changed = false;
}

void LanguageTableContainer::load() {
    this->m_lt->data().clear();

    rust::Vec<DirEntry> list = this->m_ds.list_dir(TO_RUST_STR(this->m_path));

    unimplemented();

    // move en_us to the beginning
//    for (int i = 0; i < list.size(); i++) {
//        DirEntry entry = list[i];
//        qDebug() << entry.m_path.to_string() << entry.m_path.file_stem() << entry.m_path.extension();
//        if (entry.m_path.extension() == "json" && entry.is_file) {
//            QString lang = entry.m_path.file_stem();
//            if (lang == "en_us") {
//                list.removeAt(i);
//                list.insert(0, entry);
//                break;
//            }
//        }
//    }
//
//    for (auto entry: list) {
//        if (entry.m_path.extension() == "json" && entry.is_file) {
//            QString lang = entry.m_path.file_stem();
//            this->m_lt->data().add_language(lang);
//            QSharedPointer<QIODevice> dev = entry.ref.open();
//            dev->open(QIODevice::ReadOnly | QIODevice::Text);
//            QJsonParseError err;
//            auto doc = QJsonDocument::fromJson(dev->readAll(), &err);
//
//            // TODO actually show errors
//            if (err.error != QJsonParseError::NoError) continue;
//            if (!doc.isObject()) continue;
//
//            QJsonObject object = doc.object();
//            for (QString key: object.keys()) {
//                QJsonValueRef value = object[key];
//                if (!value.isString()) continue;
//                this->m_lt->data().insert(lang, key, value.toString());
//            }
//        }
//    }
//
//    m_persistent = true;
//    m_changed = false;
//    emit m_lt->layoutChanged();
}

void LanguageTableContainer::on_changed() {
    m_changed = true;
    emit changed();
}
