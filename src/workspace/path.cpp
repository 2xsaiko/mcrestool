#include "path.h"

#include <utility>
#include <QDebug>

Path::Path() : m_inner() {
}

Path::Path(const Path& that) : m_inner(that.m_inner) {
}

Path::Path(QString spec) : m_inner(std::move(spec)) {
}

Path::Path(const char* spec) : Path(QString(spec)) {
}

Path Path::parent() const {
    auto iter = this->components();

    iter.skip_back(1);

    return iter.to_path();
}

Path Path::join(const Path& right) const {
    Path copy(*this);
    copy.push(right);
    return copy;
}

void Path::push(const Path& right) {
#ifndef _WIN32
    if (right.is_absolute()) {
        *this = right;
    } else {
        auto iter = right.components();
        if (iter.peek().type == PATHCOMP_CUR_DIR) {
            iter.skip(1);
        }

        if (!iter.is_empty()) {
            if (!this->m_inner.endsWith('/')) {
                this->m_inner += '/';
            }

            this->m_inner += iter.to_string();
        }
    }
#endif
}

void Path::push_raw(const QString& spec) {
    assert(!spec.isEmpty());
    assert(!spec.contains('/'));

    if (this->m_inner.isEmpty()) {
        this->m_inner = spec;
    } else {
        if (!this->m_inner.endsWith('/')) {
            this->m_inner += '/';
        }

        this->m_inner += spec;
    }
}

PathComponents Path::components() const {
    return PathComponents(this->m_inner);
}

QString Path::file_name() const {
    auto iter = this->components();
    auto comp = iter.next_back();

    if (comp.type == PATHCOMP_NORMAL) {
        return comp.text;
    }

    return QString();
}

const QString& Path::to_string() const {
    return this->m_inner;
}

bool Path::is_absolute() const {
    if (this->is_null()) return false;

#ifndef _WIN32
    return this->m_inner.startsWith('/');
#endif
}

bool Path::is_null() const {
    return this->m_inner.isNull();
}

PathComponent::PathComponent() : type(PATHCOMP_NULL) {
}

PathComponent::PathComponent(const QStringRef& text) {
    if (text.isEmpty()) {
        this->type = PATHCOMP_NULL;
    } else if (text == ".") {
        this->type = PATHCOMP_CUR_DIR;
    } else if (text == "..") {
        this->type = PATHCOMP_PARENT_DIR;
    } else if (text == "/") {
        this->type = PATHCOMP_ROOT;
    } else {
        this->type = PATHCOMP_NORMAL;
        this->text = text.toString();
    }
}

bool PathComponent::is_null() const {
    return this->type == PATHCOMP_NULL;
}

PathComponents::PathComponents(const Path& path) :
    m_inner(path.to_string()),
    m_left_end(0),
    m_right_end(this->m_inner.length()) {

    // initial clean up end of path
    this->peek_back();
}

PathComponent PathComponents::peek() {
    if (this->is_empty()) return PathComponent();

    int next = this->next_field(this->m_left_end);
    QStringRef text = this->m_inner.midRef(this->m_left_end, next - this->m_left_end);

    return PathComponent(text);
}

PathComponent PathComponents::peek_back() {
    PathComponent result;
    int last_right_end = this->do_skip_back(this->m_right_end);
    int last_safe_right_end = last_right_end;

    while (true) {
        int next = this->next_field_back(last_right_end);
        QStringRef text = this->m_inner.midRef(next, last_right_end - next);

        PathComponent pc(text);

        if (pc.type == PATHCOMP_CUR_DIR) {
            last_safe_right_end = last_right_end;
            last_right_end = this->do_skip_back(next);
            result = pc;
        } else if (pc.type == PATHCOMP_NULL) {
            break;
        } else {
            last_safe_right_end = last_right_end;
            result = pc;
            break;
        }
    }

    this->m_right_end = last_safe_right_end;

    if (this->is_empty()) return PathComponent();

    return result;
}

PathComponent PathComponents::next() {
    auto v = this->peek();
    this->skip(1);
    return v;
}

PathComponent PathComponents::next_back() {
    auto v = this->peek_back();
    this->skip_back(1);
    return v;
}

void PathComponents::skip(int n) {
#ifndef _WIN32
    for (int i = 0; i < n; i++) {
        if (this->is_empty()) break;

        int next_field = this->next_field(this->m_left_end);

        // skip all following '/'s
        while (next_field < this->m_right_end && this->m_inner[next_field] == '/') {
            next_field += 1;
        }

        this->m_left_end = next_field;

        // skip any "." components
        if (this->peek().type == PATHCOMP_CUR_DIR) {
            i -= 1;
        }
    }
#endif
}

int PathComponents::next_field(int start) const {
    // this works when we're currently at '/' even though this will return
    // the same index since it will skip all slashes later
    int next_field = this->m_inner.leftRef(this->m_right_end).indexOf('/', start);

    if (next_field == 0) {
        next_field = 1;
    }

    // if we're already in the last field, set the next field to end of
    // string
    if (next_field == -1) {
        next_field = this->m_right_end;
    }

    return next_field;
}

void PathComponents::skip_back(int n) {
#ifndef _WIN32
    for (int i = 0; i < n; i++) {
        if (this->is_empty()) break;

        this->m_right_end = this->do_skip_back(this->next_field_back(this->m_right_end));
    }

    // cleanup
    this->peek_back();
#endif
}

int PathComponents::next_field_back(int start) const {
    int next_field = start;

//    // skip all trailing '/'s
//    while (next_field > 0 && this->m_inner[next_field - 1] == '/') {
//        next_field -= 1;
//    }

    // special case for /
    if (this->m_left_end < 1 && start == 1 && this->m_inner[0] == '/') {
        return 0;
    }

    next_field = this->m_inner.rightRef(this->m_inner.length() - this->m_left_end)
                     .lastIndexOf('/', next_field - this->m_inner.length() - 1) + this->m_left_end + 1;

    return next_field;
}

int PathComponents::do_skip_back(int next_field) {
    // whether the current field is anything but the root element '/';
    // tests if the left end of the current field (stored in next_field)
    // isn't a '/'
    // ^ actually that comment isn't accurate but the code seems to work lmao
    bool has_content = next_field > 0 && (next_field >= this->m_right_end || this->m_inner[next_field] != '/');

    // skip all following '/'s except for the first one in the string if
    // we're not at the root entry yet
    while (next_field > (has_content ? 1 : 0) && this->m_inner[next_field - 1] == '/') {
        next_field -= 1;
    }

    return next_field;
}

QString PathComponents::to_string() const {
    if (this->is_empty()) return QString();

    return this->m_inner.mid(this->m_left_end, this->m_right_end - this->m_left_end);
}

Path PathComponents::to_path() const {
    return this->to_string();
}

bool PathComponents::is_empty() const {
    return this->m_left_end >= this->m_right_end;
}
