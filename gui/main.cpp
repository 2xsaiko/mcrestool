#include <QApplication>

#include <mainwindow.h>
#include <path.h>

#include <mcrtlib.h>

int main(int argc, char* argv[]) {
    std::string s("C++");
    say_hi_to_rust(s);

    QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
    QCoreApplication::setAttribute(Qt::AA_UseHighDpiPixmaps);

    QApplication app(argc, argv);
    MainWindow w;
    w.center();
    w.show();

    return QApplication::exec();
}
