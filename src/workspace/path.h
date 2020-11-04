#ifndef MCRESTOOL_PATH_H
#define MCRESTOOL_PATH_H

#include <QString>
#include <QStringRef>

struct PathComponent;
class PathComponents;

class Path {

public:
    Path();

    Path(const Path& that);

    Path(QString spec);

    Path(const char* spec);

    [[nodiscard]] Path parent() const;

    [[nodiscard]] Path join(const Path& right) const;

    void push(const Path& right);

    [[nodiscard]] PathComponents components() const;

    [[nodiscard]] QString file_name() const;

    [[nodiscard]] const QString& to_string() const;

    [[nodiscard]] bool is_absolute() const;

    [[nodiscard]] bool is_null() const;

private:
    QString m_inner;

};

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

    [[nodiscard]] bool is_null() const;

    PathComponentType type;
    QString text;
};

class PathComponents {

public:
    explicit PathComponents(const Path& path);

    PathComponent peek();

    PathComponent peek_back();

    PathComponent next();

    PathComponent next_back();

    void skip(int n);

    void skip_back(int n);

    [[nodiscard]] QString to_string() const;

    [[nodiscard]] Path to_path() const;

    [[nodiscard]] bool is_empty() const;

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
