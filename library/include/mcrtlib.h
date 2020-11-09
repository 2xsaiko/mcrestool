#ifndef MCRESTOOL_MCRTLIB_H
#define MCRESTOOL_MCRTLIB_H

#include <lib.rs.h>

#if defined(_MSC_VER)
#   define EXPORT __declspec(dllexport)
#   define IMPORT __declspec(dllimport)
#elif defined(__GNUC__)
#   define EXPORT __attribute__((visibility("default")))
#   define IMPORT
#else
#   define EXPORT
#   define IMPORT
#   pragma warning Unknown dynamic link import/export semantics.
#endif

#ifdef MCRTLIB_BUILD
#   define MCRTLIB_PUBLIC EXPORT
#else
#   define MCRTLIB_PUBLIC IMPORT
#endif

namespace mcrtlib {
}

#endif //MCRESTOOL_MCRTLIB_H
