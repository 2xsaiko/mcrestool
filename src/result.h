#ifndef MCRESTOOL_RESULT_H
#define MCRESTOOL_RESULT_H

#include <optional>

using std::optional;

template<typename T, typename E>
union ResultData {
    T value;
    E error;
};

template<typename T, typename E>
class Result {

public:
    static Result<T, E> Ok(T value) {
        Result<T, E> r;
        r._is_err = false;
        r.data = value;
        return r;
    }

    static Result<T, E> Err(E error) {
        Result<T, E> r;
        r._is_err = true;
        r.data = error;
        return r;
    }

    optional<T> ok() {
        if (!_is_err) {
            return data.value;
        } else {
            return optional<T>();
        }
    }

    optional<E> err() {
        if (_is_err) {
            return data.value;
        } else {
            return optional<T>();
        }
    }

    bool is_ok() {
        return !_is_err;
    }

    bool is_err() {
        return _is_err;
    }

private:
    bool _is_err;
    ResultData<T, E> data;

};

template<typename T, typename E>
Result<T, E> Ok(T value) {
    return Result<T, E>::Ok(value);
}

template<typename T, typename E>
Result<T, E> Err(E error) {
    return Result<T, E>::Err(error);
}

#endif //MCRESTOOL_RESULT_H
