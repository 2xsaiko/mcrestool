#include "fstree.h"
#include "workspace.h"

#include <QDebug>

FsTreeEntry::FsTreeEntry(const FsRef& ref, WorkspaceRoot* root, FsTreeEntry* parent) : QObject(parent), m_ref(ref), m_parent(parent), m_root(root) {

}

void FsTreeEntry::refresh() {
    switch (this->m_ref.type()) {
        case FSREF_NORMAL:
            this->refresh_as_normal();
            break;
        case FSREF_ZIP:
            this->m_root->tree()->refresh_as_zip();
            break;
    }
}

const FsRef& FsTreeEntry::ref() const {
    return this->m_ref;
}

QString FsTreeEntry::file_name() const {
    return this->m_ref.file_name();
}

FileType FsTreeEntry::file_type() const {
    return this->m_type;
}

FsTreeEntry* FsTreeEntry::parent() {
    return this->m_parent;
}

WorkspaceRoot* FsTreeEntry::root() {
    return this->m_root;
}

int FsTreeEntry::children_count() const {
    return this->m_children.length();
}

int FsTreeEntry::index_of(FsTreeEntry* child) const {
    return this->m_children.indexOf(child);
}

FsTreeEntry* FsTreeEntry::by_index(int child) {
    return this->m_children[child];
}

FsTreeEntry* FsTreeEntry::by_name(const QString& name) {
    for (FsTreeEntry* entry: this->m_children) {
        if (entry->file_name() == name) {
            return entry;
        }
    }
    
    return nullptr;
}

void FsTreeEntry::refresh_as_zip() {
    this->m_type = this->m_ref.file_type();

    QList<DirEntry> list = this->m_ref.read_dir_recursive();
    QMap<Path, int> file_count;

    qDebug() << "--- read_dir_recursive end ---";

    for (const auto& entry: list) {
        qDebug() << (entry.is_dir ? " DIR" : "FILE") << entry.path.to_string();
    }

    qDebug() << "--- dir list end ---";

    Path current_dir;
    FsTreeEntry* current = this;

    bool changed = false;
    int i = 0;

    for (const auto& entry: list) {
        qDebug() << "processing" << entry.path.to_string();

        if (entry.path.to_string() == "net/minecraft/client") {
            printf("");
        }

        Path parent = entry.path.parent();

        if (current_dir != parent) {
            qDebug() << "chdir" << current_dir.to_string() << parent.to_string();

            while (current->m_children.length() > i) {
                current->m_children.removeLast();
            }

            if (changed) {
                emit current->children_changed();
            }

            file_count[current_dir] = i;

            auto iter = parent.components();
            current = this;
            current_dir = parent;
            changed = false;
            i = file_count[current_dir];

            while (!iter.is_empty()) {
                PathComponent pc = iter.next();
                if (pc.type != PATHCOMP_NORMAL) continue;

                qDebug() << current->ref().path().to_string() << pc.to_string();
                for (FsTreeEntry* entry: current->m_children) {
                    qDebug() << entry->file_name();
                }

                current = current->by_name(pc.to_string());

                assert(current != nullptr);
            }
        }

        if (current->m_children.length() <= i) {
            current->m_children += new FsTreeEntry(entry.ref, current->m_root, current);
            changed = true;
        } else {
            QString name = current->m_children[i]->file_name();

            if (entry.path.file_name() != name) {
                while (current->m_children.length() > i && entry.path.file_name() > name) {
                    current->m_children.removeAt(i);
                }

                current->m_children.insert(i, new FsTreeEntry(entry.ref, current->m_root, current));
                changed = true;
            }
        }

        i++;
    }

    while (current->m_children.length() > i) {
        current->m_children.removeLast();
    }

    if (changed) {
        emit current->children_changed();
    }
}

void FsTreeEntry::refresh_as_normal() {
    this->m_type = this->m_ref.file_type();

    QList<DirEntry> list = this->m_ref.read_dir();

    bool changed = false;
    int i = 0;

    for (const auto& entry: list) {
        if (this->m_children.length() <= i) {
            this->m_children += new FsTreeEntry(entry.ref, this->m_root, this);
            changed = true;
        } else {
            QString name = this->m_children[i]->file_name();

            if (entry.path.file_name() != name) {
                while (this->m_children.length() > i && entry.path.file_name() > name) {
                    this->m_children.removeAt(i);
                }

                this->m_children.insert(i, new FsTreeEntry(entry.ref, this->m_root, this));
                changed = true;
            }
        }

        i++;
    }

    while (this->m_children.length() > i) {
        this->m_children.removeLast();
    }

    if (changed) {
        emit children_changed();
    }

    for (auto c: this->m_children) {
        c->refresh_as_normal();
    }
}
