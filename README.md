# ShareXin  

[![Crates.io](https://img.shields.io/crates/v/sharexin.svg?)](https://crates.io/crates/sharexin)
[![Github Stars](https://img.shields.io/github/stars/thebitstick/ShareXin.svg?)](https://github.com/thebitstick/ShareXin)
[![Crates.io Downloads](https://img.shields.io/crates/d/sharexin.svg?)](https://crates.io/crates/sharexin)  

## Screenshots
![ShareXin](https://raw.githubusercontent.com/thebitstick/ShareXin/master/dialog.png)  

## Requirements
* Unix-like system
* scrot (on non-Gnome/KDE x11 desktops)
* feh
* [t](https://github.com/sferik/t)
* [toot](https://github.com/ihabunek/toot)

## Features
* Uploads to Twitter and Mastodon and Imgur
* Allows taking screenshots and saving them to files
* Notifications via dbus
* GUI works with GTK
* Screenshotting works with X11 and Wayland (on supported desktops)
* Saves screenshots to a folder in your Pictures directory

### Desktop support
- Unity desktop
- Gnome desktop
- Budgie desktop
- Cinnamon desktop
- Plasma desktop
- Sway desktop
- General X11 desktop with scrot

## `--help`

```bash
sharexin 0.6.0
Usage: sharexin <options> [destination] [destination options/image options] [FILE]

Options:
  -h, --help	Display this help message and exit
  -V, --version	Print version info and exit
  -U, --upgrade	Check for new updates

Image Options:
  area		Grab an area of the screen instead of the entire screen
  window	Grab the current window instead of the entire screen
  full		Grab the entire screen
  file		Use a file

Destinations:
  toot		Upload to Mastodon (uses "toot")
  tweet		Upload to Twitter (uses "t")
  imgur		Upload to Imgur

Twitter Options:
  auth		Authenticate with Twitter

Mastodon Options:
  auth		Authenticate with Mastodon

Examples:
  sharexin toot
  sharexin tweet full
  sharexin toot area
  sharexin imgur file [FILE]
  sharexin tweet auth
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

## Compiling

### Dependencies for compiling
* xorg-dev
* gtk3
* cairo
* libnotify
* pango
* gdk-pixbuf2
* atk
* openssl
* libcurl4
* cc

### Ubuntu 17.04 dependencies  
* libgtk-3-dev
* libcairo2-dev
* libnotify-dev
* libpango1.0-dev
* libgdk-pixbuf2.0-dev
* libatk1.0-dev
* libssl1.0-dev
* libssl-dev
* libcurl4-openssl-dev
* build-essential

### Debian Stable dependencies
* libgtk-3-dev
* libcairo2-dev
* libnotify-dev
* libpango1.0-dev
* libgdk-pixbuf2.0-dev
* libatk1.0-dev
* libssl-dev
* libcurl4-openssl-dev

### Fedora 26 dependencies
* gtk3-devel
* cairo-devel
* libnotify-devel
* pango-devel
* gdk-pixbuf2-devel
* atk-devel
* openssl-devel
* libcurl-devel
* libcurl

### openSUSE Leap 42.3 dependencies
* gtk3-devel
* cairo-devel
* libnotify-devel
* pango-devel
* gdk-pixbuf-devel
* atk-devel
* libopenssl-devel
* libcurl-devel
* libcurl4
* make
* gcc

### openSUSE Tumbleweed dependencies
* gtk3-devel
* cairo-devel
* libnotify-devel
* pango-devel
* gdk-pixbuf-devel
* atk-devel
* libopenssl-devel
* libcurl-devel
* libcurl4
* make
* gcc

### FreeBSD 11 dependencies
* openssl-devel
* gmake
* gcc

### macOS Sierra dependencies
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

### Compiling (via Github)
1. `git clone https://github.com/ShareXin/ShareXin/`  
2. `cargo install`   

### Compiling (via Crates.io)
1. `cargo install sharexin`  

## Changelog
#### [0.6.0] - 2017-08-25
- macOS tested, Imgur works, `t` and `toot` should work if you have them
- Updated "upgrade" and error messages
- Figured out libcurl issue on some older systems
- Bug fixes

#### [0.5.9] - 2017-08-20
- ImageMagick integrated
- `killall vim` finally removed (kinda)!

#### [0.5.8] - 2017-08-20
- Imgur link now copied to clipboard _(should work on Wayland?)_
- Better desktop recognition
- Bug fixes

#### [0.5.7] - 2017-08-19
- Less repetitive code
- Area screenshots (except for KDE and Mac) use two separate files,
the frozen fullscreen and the part of it you select
- Fixed bug where Error 29 would always print (kind of a bug)

#### [0.5.6] - 2017-08-19
- Removes temporary file after sending it
- Budgie Desktop support
- Ubuntu Unity Desktop support
- Theoretical Mac support

#### [0.5.5] - 2017-08-18
- Added Polish translation
- Character count acts like ShareX, counts down from 140 or 500 (depending on destination)
- "File Saved" notification now includes image saved

#### [0.5.4] - 2017-08-17
- Just a Cargo fix, ignore

#### [0.5.3] - 2017-08-15
- Now packaging `t` (should work)
- Added `auth` option for Twitter and Mastodon

#### [0.5.2] - 2017-08-15
- GTk fixes
- Notification for error messages
- Bug fixes
- Now using AppImage for releases

#### [0.5.1] - 2017-08-14  
- No more wildcards in getting modules
- Separated `language.rs`
- Now using scrot rather than maim
- `.kill()` exists for `Command`, no longer using killall feh
- Added Fatal error messages
- Cinnamon desktop support

#### [0.5.0] - 2017-08-13
- Rewrite of Language message system, now using templates, extremely easy to add more languages
- Bug fixes
