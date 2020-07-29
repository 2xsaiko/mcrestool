#ifndef MCRESTOOL_DIRENTRY_H
#define MCRESTOOL_DIRENTRY_H

#include "fsref.h"

#include <QString>

struct WSDirEntry {
    bool is_file;
    bool is_dir;
    bool is_symlink;
    QString file_name;
    FsRef real_path;
};

#endif //MCRESTOOL_DIRENTRY_H
