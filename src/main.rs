use cxx_qt_lib::QGuiApplication;
use cxx_qt_lib::QQmlApplicationEngine;
use cxx_qt_lib::{QString, QUrl};

mod gui;
mod widgets;

fn main() {
    let mut app = QGuiApplication::new();

    let mut window = gui::QWindow::new();
    window.as_mut().unwrap().show();

    let mut window = widgets::QMainWindow::new();
    window.as_mut().unwrap().show();

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
