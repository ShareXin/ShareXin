# ShareXin  

[![Github Stars](https://img.shields.io/github/stars/thebitstick/ShareXin.svg?)](https://github.com/thebitstick/ShareXin)
[![Crates.io](https://img.shields.io/crates/v/sharexin.svg)](https://crates.io/crates/sharexin)

## Screenshots
![ShareXin](https://raw.githubusercontent.com/thebitstick/ShareXin/master/dialog.png)  

## Requirements
* Linux or FreeBSD
* rustc 1.28.0^
* scrot (on non-GNOME/Plasma X11 desktops)
* feh
* [t](https://github.com/thebitstick/t)
* [toot](https://github.com/ihabunek/toot)

## Features
* Takes screenshots
* Uploads to Twitter, Mastodon, and Imgur
* Saves screenshots to your Pictures
* Notifications
* GTK Dialog for entering a message with a tweet or toot
* Designed with Wayland in mind

### Desktop support
- Unity desktop
- GNOME desktop
- Budgie desktop
- Cinnamon desktop
- KDE Plasma desktop
- Sway desktop
- X11 DE

### Tested on
- Ubuntu 18.04
> - Debian Stable
> - Fedora 28
> - Arch Linux
> - FreeBSD with Xfce
> - TrueOS
> - GhostBSD

## `--help`

```shell
sharexin 0.6.5
TheBitStick <thebitstick@tfwno.gf>
ShareX for Linux and FreeBSD

USAGE:
    sharexin [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Display this help message and exit
    -U, --upgrade    Check for new updates
    -v, --version    Print version info and exit

SUBCOMMANDS:
    imgur    Upload to Imgur
    toot     Upload to Mastodon
    tweet    Upload to Twitter

```  

### Submit more languages using this [form](https://goo.gl/forms/rNx4yAB9KM2fDXDG3)
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

#### Debian Stable dependencies
* libgtk-3-dev
* libcairo2-dev
* libpango1.0-dev
* libgdk-pixbuf2.0-dev
* libatk1.0-dev
* libssl-dev
* libcurl4-openssl-dev
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
* dbus-1 [(don't know where to get it, won't compile right now)](https://forums.freebsd.org/threads/6191/)
* other deps

### Compiling
1. `git clone https://github.com/ShareXin/ShareXin/`  
2. `cargo install`

## Changelog
#### [0.6.6] - 2018-09-27
- Two versions in one day why
- More testing done
- Twitter and Mastodon are now using native APIs

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
