// #include "cxx-qt-lib-extras/qapplication.h"

#include "qmainwindow.h"

namespace rust
{
    namespace cxxqtlib2
    {

        ::std::unique_ptr<QMainWindow> qmainwindowNew()
        {
            auto ptr = ::std::make_unique<QMainWindow>();
            Q_ASSERT(ptr != nullptr);

            return ptr;
        }
    }
}
