<img alt="Redox" height="190" src="img/logo.png">

**Redox** is a operating system written in pure Rust, designed to be modular and secure. The development blog can be found at http://www.redox-os.org.

Docs can be found [here](http://ticki.github.io/redocs/redox/).

Please make sure you use the **latest nightly** of `rustc` before building (for more troubleshooting, see "Help! Redox won't compile!").

![travis](https://api.travis-ci.org/redox-os/redox.svg)

## What it looks like

<img alt="Redox" height="170" src="img/screenshots/Desktop.png">
<img alt="Redox" height="170" src="img/screenshots/Fancy_opacity.png">
<img alt="Redox" height="170" src="img/screenshots/File_manager.png">
<img alt="Redox" height="170" src="img/screenshots/Sodium_v1.png">
<img alt="Redox" height="170" src="img/screenshots/Boot.png">
<img alt="Redox" height="170" src="img/screenshots/start.png">


## Help! Redox won't compile!

Sometimes things goes wrong when compiling. Try the following before opening an issue:

1.  Run `make clean`.
2.  Run `git clean -X -f -d`
3.  Make sure you got **the latest version of Rust nightly!** (you can use `multirust` for managing Rust versions).
4.  Update **LLVM**, **GNU Make**, **nasm** and **QEMU/VirtualBox**.
5.  Pull the upstream master branch (`git remote add upstream git@github.com:redox-os/redox.git; git pull upstream master`).

and then rebuild!

## Cloning the repository

Make sure you get submodules when you clone the repository.
```bash
git clone --recursive
```

If you already have a copy of the repository locally without submodules, you
can download them with: 
```bash
git submodule update --init
```

## Building on Debian/Ubuntu
- Run the setup script and enter your password when prompted (to install Rust compiler and its dependencies)
```bash
cd setup
./ubuntu.sh
./binary.sh
```
- Make the project
```bash
make all
```

## Running on Debian/Ubuntu
- Install VirtualBox
```bash
sudo apt-get install virtualbox
```
- Run VirtualBox
```bash
make virtualbox
```

## Running on Debian/Ubuntu (Qemu, Advanced)
- Install Qemu
```bash
sudo apt-get install qemu-system-x86 qemu-kvm
```
- Run Qemu
```bash
make qemu
```
## Building on Arch Linux
- Run the setup script and enter your password when prompted (to install the Rust compiler and its dependencies)
```bash
cd setup
./arch.sh
./binary.sh
```
- Make the project
```bash
make
```
## Running on Arch Linux
- Virtualbox was completely setup as part of the script.
- Run Virtualbox
```bash
make virtualbox
```


## Running on Arch Linux (Qemu)

- Install Qemu
```bash
$ sudo pacman -S qemu
```
- Run redox
```bash
$ make qemu
```

## Building on Fedora
- Run the setup script and enter your password when prompted (to install Rust compiler and its dependencies)
```bash
cd setup
./fedora.sh
./binary.sh
```
- Make the project
```bash
make all
```
## Running on Fedora (Qemu, Advanced)
- Install Qemu
```bash
sudo yum install qemu-system-x86 qemu-kvm
```
- Run Qemu
```bash
make qemu
```

## Building and Running on NixOS

```bash
nix-shell setup/dev-env.nix
make all
make qemu
```

## Building on OS X
- Install MacPorts or Homebrew
- Run the setup script and enter your password when prompted (to install Rust compiler and its dependencies)
```bash
cd setup
# MacPorts
./osx-macports.sh
# Homebrew
./osx-homebrew.sh
./binary.sh
```
- Make the project
```bash
make all
```

## Running on OS X
- Install VirtualBox from https://www.virtualbox.org/wiki/Downloads
- Make sure it is installed for all users, in /Applications/ or edit the Makefile VBM path
- Run VirtualBox
```bash
make virtualbox
```

## Building on Windows
- Download and install the latest 32-bit Rust nightly from http://www.rust-lang.org/install.html
- The direct link to the 32-bit nightly is https://static.rust-lang.org/dist/rust-nightly-i686-pc-windows-gnu.msi
- Open the Rust nightly shell in the redox repository
```bash
make all
```

## Running on Windows
- Install VirtualBox from https://www.virtualbox.org/wiki/Downloads
- Make sure to install to C:\Program Files\Oracle\VirtualBox or edit the Makefile VBM path
- Run VirtualBox
```bash
make virtualbox
```
