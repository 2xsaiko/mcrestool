#ifndef MCRESTOOL_MCRTLIB_H
#define MCRESTOOL_MCRTLIB_H

#include <string>

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

MCRTLIB_PUBLIC void say_hi_to_rust(std::string& str);

#endif //MCRESTOOL_MCRTLIB_H
