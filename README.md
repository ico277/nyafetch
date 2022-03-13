# Nyafetch
A neofetch alike program that shows hardware and distro information written in rust.
# Compiling from source
## Dependencies
 rust, libpci, clang (for rust to compile c)
 
 ### Linux
**Arch(pacman):** pciutils clang

**Debian(apt):** libpci-dev clang

### Windows
*Coming soon*

More instructions as to how to install rust [*here*](https://www.rust-lang.org/tools/install).

## Installation
Either download a build [*here*](https://github.com/ico277/nyafetch/releases/latest) or compile it yourself.

## Compilation

### Building
```bash
$ make buld
```
### Installing
```bash
# make install
```
Note: You can use `make install PREFIX=<prefix>` to change the prefix.
