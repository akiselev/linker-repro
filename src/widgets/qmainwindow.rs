use core::pin::Pin;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("widgets/qmainwindow.h");
    }

    unsafe extern "C++Qt" {
        include!(<QtWidgets/QMainWindow>);
        type QMainWindow;

        fn show(self: Pin<&mut QMainWindow>);
    }

    #[namespace = "rust::cxxqtlib2"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qmainwindow_new"]
        fn qmainwindowNew() -> UniquePtr<QMainWindow>;
    }
}

pub use ffi::QMainWindow;

impl QMainWindow {
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::qmainwindow_new()
    }
}
