use core::pin::Pin;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("gui/qwindow.h");
    }

    unsafe extern "C++Qt" {
        include!(<QtGui/QWindow>);
        type QWindow;

        fn show(self: Pin<&mut QWindow>);
    }

    #[namespace = "rust::cxxqtlib2"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qwindow_new"]
        fn qwindowNew() -> UniquePtr<QWindow>;
    }
}

pub use ffi::QWindow;

impl QWindow {
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::qwindow_new()
    }
}
