#ifndef MCRESTOOL_MCRTLIBD_H
#define MCRESTOOL_MCRTLIBD_H

namespace mcrtlib::ffi {
    class TreeChangeSubscriberPrivate;

    class TreeChangeSubscriber {

    public:
        virtual ~TreeChangeSubscriber() = 0;

        virtual void pre_insert() const = 0;

        virtual void post_insert() const = 0;

        virtual void pre_remove() const = 0;

        virtual void post_remove() const = 0;

    };

    inline void tcs_pre_insert(const TreeChangeSubscriber& s) {
        s.pre_insert();
    }

    inline void tcs_post_insert(const TreeChangeSubscriber& s) {
        s.post_insert();
    }

    inline void tcs_pre_remove(const TreeChangeSubscriber& s) {
        s.pre_remove();
    }

    inline void tcs_post_remove(const TreeChangeSubscriber& s) {
        s.post_remove();
    }

    inline void tcs_set_container(TreeChangeSubscriber& s) {

    }
}

#endif //MCRESTOOL_MCRTLIBD_H
