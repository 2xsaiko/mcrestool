#include <mcrtlib.h>
#include <lib.rs.h>

void say_hi_to_rust(std::string& str) {
    mcrestool::lib::hello(str);
}