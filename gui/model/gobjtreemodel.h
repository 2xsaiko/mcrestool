#ifndef MCRESTOOL_GOBJTREEMODEL_H
#define MCRESTOOL_GOBJTREEMODEL_H

#include "rustitemmodel.h"

class GameObjectTreeModel : public RustItemModelBase {

public:
    GameObjectTreeModel(QObject* parent = nullptr);

    ~GameObjectTreeModel() override;

};

#endif //MCRESTOOL_GOBJTREEMODEL_H
