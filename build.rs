use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    let mut builder = CxxQtBuilder::new();

    builder = builder
        .qt_module("Network")
        .qt_module("Widgets")
        .qt_module("Gui");

    /*==============================================================
                            REPRO BELOW
    ==============================================================*/
    builder = builder.file("src/widgets/qmainwindow.rs");
    builder = builder.file("src/gui/window.rs");
    /*==============================================================
                            REPRO ABOVE
    ==============================================================*/

    builder = builder.cc_builder(move |cc| {
        cc.include("src/");

        cc.file("src/gui/qwindow.cpp");
        println!("cargo:rerun-if-changed=src/gui/qwindow.cpp");

        cc.file("src/widgets/qmainwindow.cpp");
        println!("cargo:rerun-if-changed=src/widgets/qmainwindow.cpp");
    });

    builder.build();
}
