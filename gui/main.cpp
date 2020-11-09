#include <QApplication>

#include <mainwindow.h>
#include <path.h>

#include <mcrtlib.h>

int main(int argc, char* argv[]) {
    mcrtlib::ffi::DataSource source = mcrtlib::ffi::datasource_open_zip("../minecraft-1.16.2-client.jar");

    QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
    QCoreApplication::setAttribute(Qt::AA_UseHighDpiPixmaps);

    QApplication app(argc, argv);
    MainWindow w;
    w.center();
    w.show();

    return QApplication::exec();
}
