#include <QApplication>

#include <mainwindow.h>
#include <path.h>

#include <mcrtlib.h>
#include <iostream>

int main(int argc, char* argv[]) {
    mcrtlib::ffi::DataSource source = mcrtlib::ffi::datasource_open_zip("../minecraft-1.15.2-client.jar");
    rust::Vec<mcrtlib::ffi::DirEntry> vec = source.list_dir("/assets/minecraft");
    for (const mcrtlib::ffi::DirEntry& d: vec) {
        std::cout << (d.info.is_dir ? " DIR " : "FILE ") << std::string(d.file_name) << std::endl;
    }

    QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
    QCoreApplication::setAttribute(Qt::AA_UseHighDpiPixmaps);

    QApplication app(argc, argv);
    MainWindow w;
    w.center();
    w.show();

    return QApplication::exec();
}
