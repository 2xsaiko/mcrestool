#ifndef MCRESTOOL_RESDOM_H
#define MCRESTOOL_RESDOM_H

#include <optional>
#include <QString>
#include "datapack.h"
#include "respack.h"

using std::optional;

class ResourceDomain {

private:
    QString domain;
    optional<DataPack> data;
    optional<ResourcePack> resources;

};

#endif //MCRESTOOL_RESDOM_H
