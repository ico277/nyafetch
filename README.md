# Nyafetch
A neofetch alike program that shows hardware and distro information written in rust.
# Compiling from source
## Dependencies
 libpci, clang, rust (for rust to compile c)
 
 ### Linux
**Arch(pacman):** pciutils clang
**Debian(apt):** libpci-dev clang

### Windows
*Coming soon*

More instructions as to how to install rust [*here*](https://www.rust-lang.org/tools/install).

## Compilation
There are 2 ways to install Nyafetch
1. Per user in `~/.cargo/bin`
2. Globally in `/usr/bin`

### 1. Per User
```bash
$ make install
```
### 2. Globally
```bash
# make install_global
```
note: Global install needs root permission
