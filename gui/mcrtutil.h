#ifndef MCRESTOOL_MCRTUTIL_H
#define MCRESTOOL_MCRTUTIL_H

#include <QtGlobal>

#define unimplemented() Q_UNIMPLEMENTED()

#define unreachable() Q_UNREACHABLE()

template<typename T>
using optional_ref = std::optional<std::reference_wrapper<T>>;

#endif //MCRESTOOL_MCRTUTIL_H
