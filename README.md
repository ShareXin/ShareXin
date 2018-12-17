## Submit more languages using this [form](https://goo.gl/forms/rNx4yAB9KM2fDXDG3)

# ShareXin

![ShareXin](https://sharexin.github.io/img/ShareX_Still.png "sharexin tweet")

[![GitHub Stars](https://img.shields.io/github/stars/thebitstick/ShareXin.svg?)](https://github.com/thebitstick/ShareXin)
[![Crates.io](https://img.shields.io/crates/v/sharexin.svg)](https://crates.io/crates/sharexin)

# Table of contents
* [Requirements](#requirements)
* [Features](#features)
* [Desktop support](#desktop-support)
    * [Tested on](#tested-on)
* [How to Use](#how-to-use)
    * [Twitter](#twitter)
    * [Mastodon](#mastodon)
    * [Imgur](#imgur)
    * [Screenshotting](#screenshotting)
    * [Keybinding](#keybinding)
* [Language support](#language-support)
* [Installation](#installation)
    * [Ubuntu/Debian dependencies](#ubuntudebian-dependencies)
    * [Arch Linux dependencies](#arch-linux-dependencies)
    * [Fedora dependencies](#fedora-dependencies)
    * [FreeBSD dependencies](#freebsd-dependencies)
    * [OpenBSD dependencies](#openbsd-dependencies)
    * [GoboLinux Dependencies](#gobolinux-dependencies)
    * [Compiling from source](#compiling-from-source)
    * [Installing from Flatpak](#installing-from-flatpak)
* [Changelog](#changelog)


## Requirements
* Linux or BSD
* `rustc` 1.31.0 (Rust 2018)
* `feh` (only need if `spectacle` is not installed or using GNOME)

## Features
* Takes screenshots
* Uploads to Twitter, Mastodon, and Imgur
* Saves screenshots to your Pictures
* Notifications
* GTK Dialog for entering a message with a tweet or toot
* Designed with **Wayland** in mind

## Desktop support
- GNOME desktop **(with `gnome-screenshot`)**
- KDE Plasma desktop **(with `spectacle`)**
- Budgie desktop
- Cinnamon desktop
- Unity desktop
- Generic X11 DE **(with `scrot`)**

#### Tested on
- Ubuntu 18.10
- Fedora 29

# How to Use
#### Twitter
**This is a temporary measure for connecting to Twitter and will be changed in the future**  
ShareXin uses the **Ruby** Twitter tool `t` for sending Tweets.  
For non-Flatpak users, you will need to manually install the tool.  
You don't need to directly interface with `t` in either Flatpak or Native, as ShareXin can call the tool directly for authenticating and tweeting.  

To authenticate to Twitter:  
`$ sharexin tweet auth`  
And on Flatpak:  
`$ flatpak run io.github.ShareXin tweet auth`
#### Mastodon
**This is a temporary measure for connecting to Mastodon and will be changed in the future**  
ShareXin uses the **Python** Mastodon tool `toot` for sending Toots.  
For non-Flatpak users, you will need to manually install the tool.  
You don't need to directly interface with `toot` in either Flatpak or Native, as ShareXin can call the tool directly for logging in and tooting.  

To login to your Mastodon instance:  
`$ sharexin toot auth`  
And on Flatpak:  
`$ flatpak run io.github.ShareXin toot auth`
#### Imgur
ShareXin uses its own API and App to upload to Imgur, so no external tool is necessary, and you do not need to setup your own API with Imgur to use it. **(if you do wish to change it, change [this line](https://github.com/ShareXin/ShareXin/blob/0c9bd4692a72e56eb8525cebed0e507321e7d341/src/imgur.rs#L30))**
#### Screenshotting
**Flatpak is currently limited to only GNOME**  
For users of the GNOME Desktop (X11/Wayland), you should already have `GNOME Screenshot` installed by default, so you do not need to worry.  
Unity, Budgie, and Cinnamon Desktop users are recommended to use `GNOME Screenshot` for the best experience, however none of these desktops have Wayland support, so any other tool mentioned can be used, such as `Spectacle` or `scrot`  
For users of the Plasma (X11/Wayland) Desktop, you should have `Spectacle` installed in order to have the best experience with ShareXin.  
For users of any X11 desktop, you must use `scrot` for screenshotting with ShareXin.
#### Keybinding
For GNOME Users, setup a custom keyboard shortcut in Settings, and if you want a custom keybinding (say to replace **Print Screen**), navigate to this Dconf setting to change a binding. `/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/`  
For Plasma Users, you do you.  
For any other desktop, you do you.

## Language support
* English
* Français (French) by [@Eleoryth](https://twitter.com/Eleoryth)
* Español (Spanish)
* Esperanto
* 简体中文 (Simplified Chinese)
* 繁體中文 (Traditional Chinese)
* 日本語 (Japanese)
*  한국어 (Korean)
* Deutsch (German) by [@qwertxzy](https://twitter.com/qwertxzy)
* Polski (Polish) by [@Michcioperz](https://twitter.com/Michcioperz)
* Português (Portuguese) by [@pillgp](https://twitter.com/pillgp)

# Installation
### Ubuntu/Debian dependencies
**Ubuntu: Tested for 17.04/17.10/18.04/18.10**  
**Debian: Tested for 9.5.0 and GNU/kFreeBSD**  
`$ sudo apt install libgtk-3-dev libcairo2-dev libpango1.0-dev libgdk-pixbuf2.0-dev libatk1.0-dev libssl-dev libcurl4-openssl-dev libclang-dev build-essential`

### Arch Linux dependencies
`$ sudo pacman -S curl gtk3 gdk-pixbuf2 cairo glib2 openssl dbus xcb-util base-devel clang`

### Fedora dependencies
**Tested for F28/F29/F30**  
`$ sudo dnf install gtk3-devel cairo-devel pango-devel gdk-pixbuf2-devel atk-devel openssl-devel libcurl-devel clang-devel`

### FreeBSD dependencies
**Tested for 11/TrueOS/GhostBSD 18.10**  
`$ sudo pkg install security/openssl-devel gmake gcc dbus-glib devel/dbus gtk3 devel/glib20 devel/pkgconf python3 cairo pango gdk-pixbuf2 atk curl`

### OpenBSD dependencies
**Tested for 6.4**  
`$ sudo pkg_add glib-openssl gmake gcc dbus-glib dbus gtk3 glib2 python cairo pango gdk-pixbuf2 atk curl pkgconfig/pkgconf`

### GoboLinux Dependencies
**NONE**, works out of the box.

## Installing from Flatpak
**Only Tested on Fedora 29 using GNOME Wayland**  
Use `io.github.ShareXin.json`

## Compiling from source
1. `$ git clone https://github.com/ShareXin/ShareXin/`
2. `$ cargo install --path .`  

**OR**  
1. `$ cargo install sharexin`

# Changelog
#### [0.7.2] - 2018-12-16
- Makefile added
- Proper App Icons!!
- Better Table of Contents in `README.md`
- Added tutorial to `README.md`
- Tested GoboLinux compatibility (weird but cool distro)
- Desktop file works with Flatpak (not tested for native installations)
- AppStream Metadata conforms to Freedesktop and Flathub standards
- No longer using `/tmp` under Flatpak (using wrapper script)
- Bug fix where multiple "zombie" processes would stay alive (pun not intended) after sending a tweet/toot

#### [0.7.1] - 2018-12-07
- Fixed TMPDIR in Flatpak (fixed an issue where it wouldn't open the screenshot you took)

#### [0.7.0] - 2018-12-07
- Flatpak works on GNOME Wayland/X11 systems
- Flatpak integrates gnome-screenshot and feh
- Fixed glib/gio issue where multiple instances of ShareXin wouldn't launch, and only one would be allowed

#### [0.6.9] - 2018-12-07
- Removed Clipboard functionality (doesn't work under Wayland)
- Flatpak fixes

#### [0.6.8] - 2018-11-02
- Moved anything in `cmd.rs` to `main.rs` (main only called cmd anyways)
- Moved anything in `save.rs` to `image.rs`
- `error.rs` is now `text.rs`, also merged with `language.rs`
- Reorganized things to accommodate for Flatpaks
- Added Flatpak!!!
- Tried to simplify a LOT of code
- `error.rs` now contains some planned enums for future rewritten error handling
- Fixed bug where some errors were not found in YAML files for some reason
- Fixed bug where because the `ShareXin` folder in pictures was already created, it attempted to display an error, which crashed for some reason
-
- Slightly modified Headerbar
- "Sent to Twitter" and "Sent to Mastodon" notifications disappear from your Notifications and don't stick around in the OSD

#### [0.6.7] - 2018-10-21
- Colours for character count on message popup match colours on Twitter Web
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
- Removed some strange and unnecessary lines
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
  - Now simply checks which software is installed, in a preferred order
- `t` and `toot` are still required for use with Twitter and Mastodon, but this WILL be addressed in 0.6.6
  - Native rust Twitter and Mastodon APIs will be added, with hopefully the same functionality.
