#ifndef MCRESTOOL_PATH_H
#define MCRESTOOL_PATH_H

#include <QString>
#include <QStringRef>

// TODO: apparently std::filesystem::path is a thing... whoops

struct PathComponent;
class PathComponents;

class Path {

public:
    Path();

    Path(QString spec);

    Path(const char* spec);

    [[nodiscard]] Path parent() const;

    [[nodiscard]] Path join(const Path& right) const;

    void push(const Path& right);

    bool pop();

    [[nodiscard]] Path strip_prefix(const Path& base) const;

    [[nodiscard]] bool starts_with(const Path& base) const;

    [[nodiscard]] bool ends_with(const Path& child) const;

    [[nodiscard]] PathComponents components() const;

    [[nodiscard]] QString file_name() const;

    [[nodiscard]] QString file_stem() const;

    [[nodiscard]] QString extension() const;

    [[nodiscard]] const QString& to_string() const;

    [[nodiscard]] bool is_absolute() const;

    [[nodiscard]] bool is_null() const;

    [[nodiscard]] bool is_empty() const;

private:
    QString m_inner;

};

inline bool operator==(const Path& left, const Path& right) {
    return left.to_string() == right.to_string();
}

inline bool operator!=(const Path& left, const Path& right) {
    return !(left == right);
}

inline bool operator<(const Path& left, const Path& right) {
    return left.to_string() < right.to_string();
}

inline uint qHash(const Path& path, uint seed = 0) {
    return qHash(path.to_string(), seed);
}

enum PathComponentType {
    PATHCOMP_NULL,
    PATHCOMP_ROOT,
    PATHCOMP_CUR_DIR,
    PATHCOMP_PARENT_DIR,
    PATHCOMP_NORMAL,
#ifdef _WIN32
    PATHCOMP_PREFIX,
#endif
};

struct PathComponent {
    PathComponent();

    explicit PathComponent(const QStringRef& spec);

    [[nodiscard]] QString to_string() const;

    [[nodiscard]] bool is_null() const;

    PathComponentType type;
    QString text;
};

inline bool operator==(const PathComponent& left, const PathComponent& right);

class PathComponents {

public:
    explicit PathComponents(const Path& path);

    PathComponents(const PathComponents& that);

    PathComponent peek();

    PathComponent peek_back();

    PathComponent next();

    PathComponent next_back();

    void skip(int n);

    void skip_back(int n);

    [[nodiscard]] QString to_string() const;

    [[nodiscard]] Path to_path() const;

    [[nodiscard]] bool is_empty() const;

    [[nodiscard]] int size() const;

private:
    [[nodiscard]] int next_field(int start) const;

    [[nodiscard]] int next_field_back(int start) const;

    int do_skip_back(int next_field);

private:
    QString m_inner;
    int m_left_end;
    int m_right_end;

};


#endif //MCRESTOOL_PATH_H
