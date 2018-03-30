# ShareXin  

[![Github Stars](https://img.shields.io/github/stars/thebitstick/ShareXin.svg?)](https://github.com/thebitstick/ShareXin)

## Screenshots
![ShareXin](https://raw.githubusercontent.com/thebitstick/ShareXin/master/dialog.png)  

## Requirements
* Unix-like system
* scrot (on non-Gnome/KDE x11 desktops)
* feh
* [t](https://github.com/thebitstick/t)
* [toot](https://github.com/ihabunek/toot)

## Features
* Uploads to Twitter and Mastodon and Imgur
* Takes screenshots and saves them to your pictures directory
* Notifications
* GTK GUI for Tweet/Toot dialog
* Screenshotting works with X11 and Wayland
* System tray icon for use without shortcuts

### Desktop support
- macOS desktop
- Unity desktop
- Gnome desktop
- Budgie desktop
- Cinnamon desktop
- Plasma desktop
- Sway desktop
- General X11 desktop with scrot

### Tested on
- Ubuntu 17.04
- Ubuntu 17.10
- Debian Stable
- Fedora 26
- Arch Linux
- FreeBSD with Xfce
- TrueOS
- openSUSE Leap 42.3
- openSUSE Tumbleweed
- macOS Sierra
- GhostBSD

## `--help`

```shell
sharexin 0.6.4
TheBitStick <thebitstick@tfwno.gf>
ShareX for Linux, FreeBSD, and macOS

USAGE:
    sharexin [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Display this help message and exit
    -t, --tray       Use system tray icon
    -U, --upgrade    Check for new updates
    -v, --version    Print version info and exit

SUBCOMMANDS:
    imgur    Upload to Imgur
    toot     Upload to Mastodon
    tweet    Upload to Twitter

```  

### Now accepting language template files in this [form](https://goo.gl/forms/rNx4yAB9KM2fDXDG3)  
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
* xorg-dev (on BSDs and Linux)
* gtk3
* cairo
* libdbus (on BSDs and Linux)
* pango
* gdk-pixbuf2
* atk (on BSDs and Linux)
* openssl (on BSDs and Linux)
* libcurl4 (on BSDs and Linux)
* cc
* libappindicator (on BSDs and Linux)
* libindicator (on BSDs and Linux)
* libdbusmenu (on BSDs and Linux)
* clang

#### Ubuntu 17.04 dependencies  
* libgtk-3-dev
* libcairo2-dev
* libpango1.0-dev
* libgdk-pixbuf2.0-dev
* libatk1.0-dev
* libssl1.0-dev
* libssl-dev
* libcurl4-openssl-dev
* build-essential
* libappindicator3-dev
* libclang-dev
* clang

#### Debian Stable dependencies
* libgtk-3-dev
* libcairo2-dev
* libpango1.0-dev
* libgdk-pixbuf2.0-dev
* libatk1.0-dev
* libssl-dev
* libcurl4-openssl-dev
* libappindicator3-dev
* libclang-dev
* clang

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
* libappindicator-gtk3

#### Fedora 26 dependencies
* gtk3-devel
* cairo-devel
* pango-devel
* gdk-pixbuf2-devel
* atk-devel
* openssl-devel
* libcurl-devel
* libcurl
* libappindicator-devel
* libappindicator-gtk3-devel
* libindicator-devel
* libindicator-gtk3-devel
* libdbusmenu-gtk3-devel
* libdbusmenu-devel
* clang-devel
* clang

#### openSUSE Leap 42.3 dependencies
* gtk3-devel
* cairo-devel
* pango-devel
* gdk-pixbuf-devel
* atk-devel
* libopenssl-devel
* libcurl-devel
* libcurl4
* make
* gcc
* libappindicator-devel
* libappindicator3-devel
* libindicator-devel
* libindicator3-devel
* libdbusmenu-glib-devel
* libdbusmenu-gtk3-devel
* llvm-clang
* libclang

#### openSUSE Tumbleweed dependencies
* gtk3-devel
* cairo-devel
* pango-devel
* gdk-pixbuf-devel
* atk-devel
* libopenssl-devel
* libcurl-devel
* libcurl4
* make
* gcc
* libappindicator-devel
* libappindicator3-devel
* libindicator-devel
* libindicator3-devel
* libdbusmenu-glib-devel
* libdbusmenu-gtk3-devel
* clang
* clang-devel
* libclang4
* libclang3_8

#### FreeBSD 11 dependencies
* openssl-devel
* gmake
* gcc
* dbus-1 [(don't know where to get it, won't compile right now)](https://forums.freebsd.org/threads/6191/)
* other deps

#### macOS Sierra dependencies
* gtk+3 (via Homebrew or via source with [jhbuild](https://wiki.gnome.org/Projects/GTK+/OSX/Building))
* xcode

### Compling tested on
- Ubuntu 17.04
- Debian Stable
- Fedora 26
- Arch Linux with i3
- FreeBSD with Xfce
- TrueOS
- openSUSE Leap 42.3
- openSUSE Tumbleweed
- macOS Sierra

### Compiling
1. `git clone https://github.com/ShareXin/ShareXin/`  
2. `cargo install`   

## Changelog
#### [0.6.4] - 2018-03-30
- Wow this project keeps going
- Added Portuguese translation

#### [0.6.3] - 2017-10-19
- Added support for Ubuntu 17.10 Gnome Desktop (congrats on upgrading to the future)

#### [0.6.2] - 2017-09-01
- System tray support (not for Mac)
- New slogan, guaranteed to work on Linux, FreeBSD, and macOS only
- Makefile and PKGBUILD
- Better command line parsing with clap-rs
- ShareXin library? I guess?

#### [0.6.1] - 2017-08-26
- XDG Directory for Pictures folder (switched my system to French, directories changed)

#### [0.6.0] - 2017-08-25
- macOS tested, Imgur works, `t` and `toot` should work if you have them
- Updated "upgrade" and error messages
- Figured out libcurl issue on some older systems
- Bug fixes
