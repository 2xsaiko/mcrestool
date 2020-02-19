#ifndef MCRESTOOL_TABLE_H
#define MCRESTOOL_TABLE_H

#include <QMap>
#include <QSet>
#include <QList>
#include <optional>

using std::optional;

template<typename C, typename R, typename V>
class Table {

public:
    void insert(const C& column, const R& row, const V& value) {
        map.insert(QPair(column, row), value);
        _columns += column;
        _rows += row;
    }

    V& get(const C& column, const R& row) {
        const QPair<C, R> pair = QPair(column, row);
        _columns += column;
        _rows += row;
        return map[pair];
    }

    optional<V> get(const C& column, const R& row) const {
        const QPair<C, R> pair = QPair(column, row);
        if (map.contains(pair)) {
            return map[pair];
        } else {
            return optional<V>();
        }
    }

    optional<V> remove(const C& column, const R& row) {
        const QPair<C, R> pair = QPair(column, row);
        if (map.contains(pair)) {
            const optional<V> value = map.take(pair);
            update_entries();
            return value;
        } else {
            return optional<V>();
        }
    }

    bool contains(const C& column, const R& row) const {
        return map.contains(QPair(column, row));
    }

    bool contains_column(const C& column) const {
        return _columns.contains(column);
    }

    bool contains_row(const R& row) const {
        return _rows.contains(row);
    }

    QList<C> columns() const {
        return _columns.values();
    }

    QList<V> rows() const {
        return _rows.values();
    }

    void clear() {
        map.clear();
        _rows.clear();
        _columns.clear();
    }

private:
    QMap<QPair<C, R>, V> map;
    QSet<C> _columns;
    QSet<R> _rows;

    void update_entries() {
        QSet<C> cols;
        QSet<R> rows;
            foreach (auto key, map.keys()) {
                cols.insert(key.first);
                rows.insert(key.second);
            }
        _columns = cols;
        _rows = rows;
    }

};

#endif //MCRESTOOL_TABLE_H
