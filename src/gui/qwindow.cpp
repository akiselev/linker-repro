// #include "cxx-qt-lib-extras/qapplication.h"

#include "qwindow.h"

namespace rust
{
    namespace cxxqtlib2
    {
        ::std::unique_ptr<QWindow> qwindowNew()
        {
            auto ptr = ::std::make_unique<QWindow>(static_cast<QWindow *>(nullptr));
            Q_ASSERT(ptr != nullptr);

            return ptr;
        }
    }
}
