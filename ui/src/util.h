#ifndef MCRESTOOL_UTIL_H
#define MCRESTOOL_UTIL_H

#include <QtGlobal>
#include <QWidget>

#define unimplemented() (qt_assert("unimplemented", __FILE__, __LINE__))

#define unreachable() (qt_assert("unreachable", __FILE__, __LINE__))

void check_for_error(QWidget* parent);

#endif //MCRESTOOL_UTIL_H
