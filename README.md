# Neotron QEMU BIOS

A [Neotron](https://github.com/neotron-compute) BIOS that runs on `qemu-system-arm`.

We use QEMU to emulate an Arm Cortex-M3 developer kit - the MPS2-AN385.

Just install `qemu-system-arm` and run:

```console
$ cargo run
```

Or you can:

```console
$ cargo build
$ qemu-system-arm -cpu cortex-m3 -machine mps2-an385 -semihosting-config enable=on,target=native -kernel ./target/thumbv7m-none-eabi/debug/neotron-qemu-bios
```

To debug, try:

```console
$ cargo build
$ qemu-system-arm -gdb tcp::3333 -S -cpu cortex-m3 -machine mps2-an385 -semihosting-config enable=on,target=native -kernel ./target/thumbv7m-none-eabi/debug/neotron-qemu-bios
```

Then, in another terminal:

```console
$ gdb ./target/thumbv7m-none-eabi/debug/neotron-qemu-bios
```

![Build Status](https://github.com/thejpster/neotron-qemu-bios/workflows/Build/badge.svg "Github Action Build Status")

![Format Status](https://github.com/thejpster/neotron-qemu-bios/workflows/Format/badge.svg "Github Action Format Check Status")

## Compatibility

This BIOS will run on QEMU when set to emulate an Arm MPS2-AN385.

## Features

* Serial output, which goes to the QEMU console.

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
