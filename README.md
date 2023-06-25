# Neotron QEMU BIOS

A [Neotron](https://github.com/neotron-compute) BIOS that runs on `qemu-system-arm`.

We use QEMU to emulate an Arm Cortex-M3 developer kit - the MPS3-AN547.

Just install `qemu-system-arm` and run:

```console
$ zcat src/disk.img.gz > src/disk.img # Unpack a fresh disk image
$ cargo run
   Compiling neotron-qemu-bios v0.1.0 (/home/user/Neotron-QEMU-BIOS)
    Finished dev [unoptimized + debuginfo] target(s) in 0.23s
     Running `qemu-system-arm -cpu cortex-m55 -machine mps3-an547 -serial stdio -kernel target/thumbv7m-none-eabi/debug/neotron-qemu-bios`
Neotron QEMU BIOS 0.1.0
Configured Serial console on Serial 0
Welcome to Neotron OS, version 0.3.3 (git:heads/lib-mode-0-g0fda2b0-dirty)!
Copyright © Jonathan 'theJPster' Pallant and the Neotron Developers, 2022
TPA: 487424 bytes @ 0x20001000

> 
```

By default, the Neotron OS console is the console where you ran QEMU. The QEMU command line is in [`.cargo/config.toml`](.cargo/config.toml).

To debug, edit the `runner` line in the [`config.toml`](./.cargo/config.toml) (see the comment above it), and then connect with GDB to boot the system:

```console
$ gdb ./target/thumbv7m-none-eabi/debug/neotron-qemu-bios
```

You can also use an ARM Corstone SSE-300 Fixed Virtual Platform (FVP), which emulates a Cortex-M55. This is a free-of-charge download from Arm at https://developer.arm.com/downloads/-/arm-ecosystem-fvps. When you run this, the UART console will appear in an `xterm` window (so make sure you have `xterm` installed ... I don't know what it does on Windows).

```console
$ zcat src/disk.img.gz > src/disk.img # Unpack a fresh disk image
$ cargo build
$ ~/FVP_Corstone_SSE-300/models/Linux64_GCC-6.4/FVP_Corstone_SSE-300_Ethos-U55 \
    -a ~/Documents/github/Neotron-QEMU-BIOS/target/thumbv7m-none-eabi/debug/neotron-qemu-bios
telnetterminal0: Listening for serial connection on port 5000
telnetterminal1: Listening for serial connection on port 5001
telnetterminal2: Listening for serial connection on port 5002
telnetterminal5: Listening for serial connection on port 5003

    Ethos-U rev 136b7d75 --- Nov 25 2021 12:05:57
    (C) COPYRIGHT 2019-2021 Arm Limited
    ALL RIGHTS RESERVED
```

![Build Status](https://github.com/thejpster/neotron-qemu-bios/workflows/Build/badge.svg "Github Action Build Status")

![Format Status](https://github.com/thejpster/neotron-qemu-bios/workflows/Format/badge.svg "Github Action Format Check Status")

## Disk Image

The samples in the disk image are from https://github.com/Neotron-Compute/Neotron-SDK/tree/a2f224840c4cd5076c767e0df2322b10bde24945.

## Compatibility

This BIOS will run on QEMU when set to emulate an Arm MPS3-AN547 (`-machine mps3-an547`), or on the Arm FVP for that platform. I guess it'll run on the real thing, but they're probably very expensive.

## Features

* Serial output, which goes to and comes from the QEMU console.

## Changelog

### Unreleased Changes ([Source](https://github.com/thejpster/neotron-qemu-bios/tree/main) | [Changes](https://github.com/thejpster/neotron-qemu-bios/compare/v0.1.0..main))

* None

### v0.1.0 ([Source](https://github.com/thejpster/neotron-qemu-bios/tree/v0.1.0)

* First release

## Licence

	Neotron-QEMU-BIOS
    Copyright (c) Jonathan 'theJPster' Pallant, 2023
    Copyright (c) The Neotron Developers, 2023

	This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be licensed as above, without
any additional terms or conditions.
