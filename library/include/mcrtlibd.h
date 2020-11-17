#ifndef MCRESTOOL_MCRTLIBD_H
#define MCRESTOOL_MCRTLIBD_H

#include <cstddef>

namespace rust {
    inline namespace cxxbridge05 {
        template<typename T>
        class Vec;
    }
}

namespace mcrtlib::ffi {
    class TreeChangeSubscriber {

    public:
        virtual ~TreeChangeSubscriber() = default;

        virtual void pre_insert(const rust::Vec<size_t>& path, size_t start, size_t end) = 0;

        virtual void post_insert(const rust::Vec<size_t>& path) = 0;

        virtual void pre_remove(const rust::Vec<size_t>& path, size_t start, size_t end) = 0;

        virtual void post_remove(const rust::Vec<size_t>& path) = 0;

    };

    inline void tcs_pre_insert(TreeChangeSubscriber& s, const rust::Vec<size_t>& path, size_t start, size_t end) {
        s.pre_insert(path, start, end);
    }

    inline void tcs_post_insert(TreeChangeSubscriber& s, const rust::Vec<size_t>& path) {
        s.post_insert(path);
    }

    inline void tcs_pre_remove(TreeChangeSubscriber& s, const rust::Vec<size_t>& path, size_t start, size_t end) {
        s.pre_remove(path, start, end);
    }

    inline void tcs_post_remove(TreeChangeSubscriber& s, const rust::Vec<size_t>& path) {
        s.post_remove(path);
    }
}

#endif //MCRESTOOL_MCRTLIBD_H
