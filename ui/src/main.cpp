#include "src/ui/mainwindow.h"
#include "src/fs/datasource.h"
#include "src/fs/archivedatasource.h"
#include <QApplication>
#include <QDebug>
#include "mcrestool_lib.h"

int main(int argc, char* argv[]) {
    it_works();
    QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
    QCoreApplication::setAttribute(Qt::AA_UseHighDpiPixmaps);

    QApplication app(argc, argv);
    MainWindow w;
    w.center();
    w.show();

    return QApplication::exec();
}
