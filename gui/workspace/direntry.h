#ifndef MCRESTOOL_DIRENTRY_H
#define MCRESTOOL_DIRENTRY_H

#include "fsref.h"

#include <QString>

struct DirEntry {
    bool is_file;
    bool is_dir;
    bool is_symlink;
    Path path;
    FsRef ref;
};

#endif //MCRESTOOL_DIRENTRY_H
