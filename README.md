# Neotron QEMU BIOS

A [Neotron](https://github.com/neotron-compute) BIOS that runs on `qemu-system-arm`.

We use QEMU to emulate an Arm Cortex-M3 developer kit - the MPS2-AN385.

Just install `qemu-system-arm` and run:

```console
$ cargo run
   Compiling neotron-qemu-bios v0.1.0 (/home/user/Neotron-QEMU-BIOS)
    Finished dev [unoptimized + debuginfo] target(s) in 0.23s
     Running `qemu-system-arm -cpu cortex-m3 -machine mps2-an385 -semihosting-config enable=on,target=native -serial stdio -kernel target/thumbv7m-none-eabi/debug/neotron-qemu-bios`
Neotron QEMU BIOS 0.1.0
Configured Serial console on Serial 0
Welcome to Neotron OS, version 0.3.3 (git:heads/lib-mode-0-g0fda2b0-dirty)!
Copyright © Jonathan 'theJPster' Pallant and the Neotron Developers, 2022
TPA: 2093056 bytes @ 0x20001000

> 
```

By default, the Neotron OS console is the console where you ran QEMU. The QEMU command line is in [`.cargo/config.toml`](.cargo/config.toml).

To debug, edit the `runner` line in the config, and then connect with GDB to boot the system:

```console
$ gdb ./target/thumbv7m-none-eabi/debug/neotron-qemu-bios
```

![Build Status](https://github.com/thejpster/neotron-qemu-bios/workflows/Build/badge.svg "Github Action Build Status")

![Format Status](https://github.com/thejpster/neotron-qemu-bios/workflows/Format/badge.svg "Github Action Format Check Status")

## Compatibility

This BIOS will run on QEMU when set to emulate an Arm MPS2-AN385.

## Features

* Serial output, which goes to and comes from the QEMU console.

## Changelog

### Unreleased Changes ([Source](https://github.com/thejpster/neotron-qemu-bios/tree/master) | [Changes](https://github.com/thejpster/neotron-qemu-bios/compare/v0.2.0...master))

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
