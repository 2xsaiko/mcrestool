#include "src/ui/mainwin.h"
#include "src/fs/datasrc.h"
#include "src/fs/arcdtsrc.h"
#include <QApplication>
#include <QDebug>

int main(int argc, char* argv[]) {
    QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
    QCoreApplication::setAttribute(Qt::AA_UseHighDpiPixmaps);

    QApplication app(argc, argv);
    MainWindow w;
    w.center();
    w.show();

    return QApplication::exec();
}
