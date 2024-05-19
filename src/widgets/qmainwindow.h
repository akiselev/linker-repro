// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <memory>

#include <QtWidgets/QMainWindow>

#include "rust/cxx.h"

namespace rust
{
    namespace cxxqtlib2
    {
        ::std::unique_ptr<QMainWindow>
        qmainwindowNew();
    }
}
