/*
 * Copyright (C) 2022 Eric Curtin <ericcurtin17@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published
 * by the Free Software Foundation; either version 2 of the licence or (at
 * your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General
 * Public License along with this library. If not, see <https://www.gnu.org/licenses/>.
 */

#pragma once

#include "ostree-bootloader.h"

G_BEGIN_DECLS

#define OSTREE_TYPE_BOOTLOADER_ABOOT (_ostree_bootloader_aboot_get_type ())
#define OSTREE_BOOTLOADER_ABOOT(inst) (G_TYPE_CHECK_INSTANCE_CAST ((inst), OSTREE_TYPE_BOOTLOADER_ABOOT, OstreeBootloaderAboot))
#define OSTREE_IS_BOOTLOADER_ABOOT(inst) (G_TYPE_CHECK_INSTANCE_TYPE ((inst), OSTREE_TYPE_BOOTLOADER_ABOOT))

typedef struct _OstreeBootloaderAboot OstreeBootloaderAboot;

GType _ostree_bootloader_aboot_get_type (void) G_GNUC_CONST;

OstreeBootloaderAboot * _ostree_bootloader_aboot_new (OstreeSysroot *sysroot);
G_END_DECLS
