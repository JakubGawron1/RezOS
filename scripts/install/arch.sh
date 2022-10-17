#!/bin/bash
set -x

# DEPENDENCIES:
# curl     - used to install rust
# make     - base build system, required
# nasm     - assembler, required
# gcc      - C compiler, used in BOOTBOOT and MAKEFILE2GRAPH, required if those are not already built
# qemu    - QEMU emulator, required for debug/run
# graphviz - dot client, used to generate log/buildflow.png
# rustup   - the recomended rust toolchain installer/manager

pacman -Sy --noconfirm curl make nasm gcc qemu graphviz mtools xorriso

curl https://sh.rustup.rs -sSf | sh -s -- -y
