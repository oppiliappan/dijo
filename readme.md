<p align="center">
  <img height="200" src="https://u.peppe.rs/lz.png">
</p>

### About

`dijo` is a habit tracker. It is curses-based, it runs in
your terminal. `dijo` is scriptable, hook it up [with
external
programs](https://github.com/NerdyPepper/dijo/wiki/Auto-Habits)
to track events without moving a finger. `dijo` is modal,
much like a certain text editor.  

### Features

 - **vim like motions**: navigate `dijo` with `hjkl`!
 - **`dijo` is modal**: different modes to view different
   stats!
 - **vim like command mode**: add with `:add`, delete with
   `:delete` and above all, quit with `:q`!.
 - **fully scriptable**: [configure `dijo` to
   track your `git` commits](https://github.com/NerdyPepper/dijo/wiki/Auto-Habits)!

### Install

To get the latest release of `dijo`, prefer installing it
via `cargo`. Unofficial packages exist for some package
managers as well. You can also browse the
[Releases](https://github.com/NerdyPepper/dijo/releases)
page for prebuilt binaries.

#### Cargo

```shell
# dijo requires rustc >= v1.42
$ rustup update

$ cargo install dijo
```
If you aren't familiar with `cargo` or Rust, read the [complete
installation](https://github.com/NerdyPepper/dijo/wiki/Install)
guide.

#### Nix

`dijo` on nixpkgs (maintained by [@Infinisil](https://github.com/Infinisil)):

```
$ nix-env -f channel:nixpkgs-unstable -iA dijo
```

#### Arch Linux

Install [`dijo-bin`](https://aur.archlinux.org/packages/dijo-bin/) or [`dijo-git`](https://aur.archlinux.org/packages/dijo-git) from the AUR.

#### Windows

```
# the default termion backend dosen't run on windows yet
$ cargo install --no-default-features --features "crossterm-backend"
```

### Usage

`dijo` has a [detailed
wiki](https://github.com/NerdyPepper/dijo/wiki/), here are
some good places to start out:

 - [Getting started](https://github.com/NerdyPepper/dijo/wiki/Getting-Started)
 - [Automatically tracking habits](https://github.com/NerdyPepper/dijo/wiki/Auto-Habits)
 - [Command reference](https://github.com/NerdyPepper/dijo/wiki/Commands)

### Gallery

Day mode, shows days of the current month:

![day.png](https://u.peppe.rs/qI.png)

Week mode, shows weekly summary for the weeks of the month:

![weekly.png](https://u.peppe.rs/HZ.png)

[![Awesome Humane Tech](https://raw.githubusercontent.com/humanetech-community/awesome-humane-tech/main/humane-tech-badge.svg?sanitize=true)](https://github.com/humanetech-community/awesome-humane-tech)
