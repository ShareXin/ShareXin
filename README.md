## Submit more languages using this [form](https://goo.gl/forms/rNx4yAB9KM2fDXDG3)
#### Flatpak currently has limited functionality, can only send to Mastodon and no screenshots

# ShareXin

![ShareXin](https://sharexin.github.io/img/ShareX_Still.png "sharexin tweet")

[![GitHub Stars](https://img.shields.io/github/stars/thebitstick/ShareXin.svg?)](https://github.com/thebitstick/ShareXin)
[![Crates.io](https://img.shields.io/crates/v/sharexin.svg)](https://crates.io/crates/sharexin)

## Requirements
* Linux or FreeBSD
* rustc 1.30.1
* *scrot* (only need if `gnome-screenshot` not installed)
* *feh* (only need if `spectacle` is not installed or using GNOME Wayland)

## Features
* Takes screenshots
* Uploads to Twitter, Mastodon, and Imgur
* Saves screenshots to your Pictures
* Notifications
* GTK Dialog for entering a message with a tweet or toot
* Designed with **Wayland** in mind

### Desktop support
- GNOME desktop
- KDE Plasma desktop
- Budgie desktop
- Cinnamon desktop
- Unity desktop
- Any X11 DE

### Tested on
- Ubuntu 18.10
- Debian 9.5.0
- Fedora 29
- Arch Linux
- GhostBSD 18.10 (FreeBSD distro)

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

#### Ubuntu 18.10 dependencies
* libgtk-3-dev
* libcairo2-dev
* libpango1.0-dev
* libgdk-pixbuf2.0-dev
* libatk1.0-dev
* libssl-dev
* libcurl4-openssl-dev
* libclang-dev
* build-essential

#### Debian 9.5.0 dependencies
* libgtk-3-dev
* libcairo2-dev
* libpango1.0-dev
* libgdk-pixbuf2.0-dev
* libatk1.0-dev
* libssl-dev
* libcurl4-openssl-dev
* libclang-dev
* build-essential

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

#### Fedora 29 dependencies
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

**OR**
1. `cargo install sharexin`

## Changelog
#### [0.6.9] - 2018-11-16
- Removed Clipboard functionality (doesn't work under Wayland)
- Flatpak fixes (Ruby is still a pain, screenshotting doesn't work cause sandboxing doesn't allow access to host's gnome-screenshot, or spectacle)

#### [0.6.8] - 2018-11-02
- Moved anything in `cmd.rs` to `main.rs` (main only called cmd anyways)
- Moved anything in `save.rs` to `image.rs`
- `error.rs` is now `text.rs`, also merged with `language.rs`
- Reorganized things to accomodate for Flatpaks
- Added Flatpak!!!
- Tried to simplify a LOT of code
- `error.rs` now contains some planned enums for future rewritten error handling
- Fixed bug where some errors were not found in YAML files for some reason
- Fixed bug where because the `ShareXin` folder in pictures was already created, it attempted to display an error, which crashed for some reason
-
- Slightly modified Headerbar
- "Sent to Twitter" and "Sent to Mastodon" notifications disappear from your Notifications and don't stick around in the OSD

#### [0.6.7] - 2018-10-21
- Colors for character count on message popup match colors on Twitter Web
- Character count on message popup turns yellow when approaching limit with 20 characters left, just like on Twitter Web
- Didn't know `unreachable!("")` was a thing, replaced some instances of `panic!()` with it
- Removed useless `error::fatal()` message
- Disabling *"Ok"* button when no text is entered or when too much text is over service limit
- Removed "File saved" notification, not really needed
- Updated dependencies
- Debian is actually compatible **(had to regress from 3_22_30 of gtk-rs to 3_18)**
- Better comments
- `--upgrade` removed due to issues with openssl
- Character count only changes when keys are pressed in the TEXT Box, not the entire window
- Made `error.rs` actually readable
- Notifications for a sent image or status tweet/toot or Imgur post only last 2 seconds

#### [0.6.6] - 2018-10-31
- More testing done
- Removed some strange and unneccesary lines
- Removed lots of duplicate code
- Replaced many if else statements with match statements
- AppImage script provided
- General bug fixes
- Split screenshotting functionality off to [screenshot-rs](https://github.com/ShareXin/screenshot-rs)
- Removed swaywm support (even back when I used it, my implementation was trash, if any this is a service to sway users)
- Removed RefCell in `dialog.rs`
- Updated Twitter character limit (not full proof for non-Latin characters)
- Heavy rewrite of functions with clearer variables
- Old Twitter API restored for functionality, will be replaced by native API in a later update

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
- `t` and `toot` are still required for use with Twitter and Mastodon, but this WILL be addressed in 0.6.6
  - Native rust Twitter and Mastodon APIs will be added, with hopefully the same functionality.
