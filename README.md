## Submit more languages using this [form](https://goo.gl/forms/rNx4yAB9KM2fDXDG3)

# ShareXin  

[![GitHub Stars](https://img.shields.io/github/stars/thebitstick/ShareXin.svg?)](https://github.com/thebitstick/ShareXin)
[![Crates.io](https://img.shields.io/crates/v/sharexin.svg)](https://crates.io/crates/sharexin)

## Requirements
* Linux or FreeBSD
* rustc 1.28.0^ (through rustup)
* *scrot* (on non-GNOME/Plasma X11 desktops)
* *feh*

> Hopefully in the future, **[Flameshot](https://github.com/lupoDharkael/flameshot)** will be prefered
> for screenshots over *gnome-screenshot* or *Spectacle*.
> It provides Wayland screenshotting with great features and looks nice
> Once this [issue](https://github.com/lupoDharkael/flameshot/issues/302) has been resolved, it will be added to [screenshot-rs](https://github.com/ShareXin/screenshot-rs).

## Features
* Takes screenshots
* Uploads to Twitter, Mastodon, and Imgur
* Saves screenshots to your Pictures
* Notifications
* GTK Dialog for entering a message with a tweet or toot
* Designed with Wayland in mind

### Desktop support
- GNOME desktop
- KDE Plasma desktop
- Budgie desktop
- Cinnamon desktop
- Unity desktop
- X11 DE

### Tested on
- Ubuntu 18.04
- Debian 9.5.0
- Fedora 28
- Arch Linux
- GhostBSD (FreeBSD distro)

## `--help`

## Language support
* English
* Français (French) by [@Eleoryth](https://twitter.com/Eleoryth)
* Español (Spanish)
* Esperanto
* 简体中文 (Simplified Chinese)
* 繁體中文 (Traditional Chinese)
* 日本語 (Japanese)
* 한국어 (Korean)
* Deutsch (German) by [@qwertxzy](https://twitter.com/qwertxzy)
* Polski (Polish) by [@Michcioperz](https://twitter.com/Michcioperz)
* Português (Portuguese) by [@pillgp](https://twitter.com/pillgp)

## Compiling

### Dependencies for compiling
* xorg-dev
* gtk3
* cairo
* libdbus
* pango
* gdk-pixbuf2
* atk
* openssl
* libcurl4
* cc
* clang

#### Ubuntu 18.04 dependencies
* libgtk-3-dev
* libcairo2-dev
* libpango1.0-dev
* libgdk-pixbuf2.0-dev
* libatk1.0-dev
* libssl-dev
* libcurl4-openssl-dev
* libclang-dev
* clang

#### Debian 9.5.0 dependencies
* libgtk-3-dev
* libcairo2-dev
* libpango1.0-dev
* libgdk-pixbuf2.0-dev
* libatk1.0-dev
* libssl-dev
* libcurl4-openssl-dev
* libclang-dev
* clang

##### Debian issues
As of Oct 5, 2018, you need these Ubuntu .deb for it to compile on Debian
- libglib2.0-0_2.55.2-1ubuntu1_amd64.deb
- libglib2.0-bin_2.55.2-1ubuntu1_amd64.deb
- libglib2.0-dev-bin_2.55.2-1ubuntu1_amd64.deb
- libglib2.0-dev_2.55.2-1ubuntu1_amd65.deb
- fontconfig-config_2.12.6-0ubuntu1_all.deb
- gir1.2-gtk-3.0_3.22.29-2ubuntu1_amd64.deb
- libgtk3-0_3.22.29-2ubuntu1_amd64.deb
- libgtk3-common_3.22.29-2ubuntu1_all.deb
- libgtk3-dev_3.22.29-2ubuntu1_amd64.deb
- wayland-protocols_1.10-1_all.deb

#### Arch Linux dependencies
* curl
* gtk3
* gdk-pixbuf2
* cairo
* glib2
* openssl
* dbus
* xcb-util
* base-devel
* clang

#### Fedora 28 dependencies
* gtk3-devel
* cairo-devel
* pango-devel
* gdk-pixbuf2-devel
* atk-devel
* openssl-devel
* libcurl-devel
* clang-devel

#### FreeBSD 11 dependencies
* openssl-devel
* gmake
* gcc
* dbus-glib
* devel/dbus
* gtk3
* devel/glib20 (via ports)
* cairo
* pango
* gdk-pixbuf2
* atk
* openssl
* curl

### Compiling
1. `git clone https://github.com/ShareXin/ShareXin/`  
2. `cargo install`

## Changelog
#### [0.6.6] - 2018-10-31
- More testing done
- Removed some strange and unneccesary lines
- Removed lots of duplicate code
- Replaced many if else statements with match statements
- AppImage script provided
- General bug fixes
- Split screenshotting functionality off to [screenshot-rs](https://github.com/ShareXin/screenshot-rs)
- Removed swaywm support (even back when I used it, my implementation was trash, if any this is a service to sway users)
- Removed RefCell in *dialog.rs*
- Updated Twitter character limit (not full proof for non-Latin characters)
- Heavy rewrite of functions with clearer variables
- Old Twitter API restored for functionality, will be replaced by native API in 0.6.7

#### [0.6.5] - 2018-09-26
- Work will continue on the project!
- Updated GNOME deps to 3_22_30 from 3_10
- Removed macOS support (how many people even used it?)
- Removed tray icon support (unneeded and unusable, GNOME doesn't even support it anymore)
- Updated various dependencies
- Removed ShareXin as a library
- Readded ShareXin to crates.io
- Added Subfolder under ShareXin in Pictures for sorting by years-month
- Rewrote method of determining screenshot selection software
  - Used to rely on hard-coded desktop variables to match with specific software
  - Now simply checks which software is installed, in a prefered order
- **t** and **toot** are still required for use with Twitter and Mastodon, but this WILL be addressed in 0.6.6
  - Native rust Twitter and Mastodon APIs will be added, with hopefully the same functionality.
