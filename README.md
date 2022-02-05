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
2. Globally in `/usr/bin` (changable using `make <subcommand> PREFIX=<custom prefix>`)

### 1. Per User
```bash
$ make install
```
### 2. Globally
```bash
# make install_global PREFIX=<prefix here>
```
note: Global install might require root permission

## Uninstall
### 1. Per User
```bash
$ make uninstall
```
### 2. Globally
```bash
# make uninstall_global PREFIX=<prefix here>
```
note: Global uninstall might require root permission
