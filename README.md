# ShareXin  

[![Crates.io](https://img.shields.io/crates/v/sharexin.svg?)](https://crates.io/crates/sharexin)
[![Github Stars](https://img.shields.io/github/stars/thebitstick/ShareXin.svg?)](https://github.com/thebitstick/ShareXin)
[![Crates.io Downloads](https://img.shields.io/crates/d/sharexin.svg?)](https://crates.io/crates/sharexin)  

## Screenshots
![Mastodon](https://raw.githubusercontent.com/thebitstick/ShareXin/master/ui-mastodon.png)
![Twitter](https://raw.githubusercontent.com/thebitstick/ShareXin/master/ui-twitter.png)  

## Requirements  
* Rust Nightly  
* xdg (probably already installed)  
* maim (on non-Gnome/KDE x11 desktops)  
* slop (on non-Gnome/KDE x11 desktops)  
* imagemagick  
* [t](https://github.com/sferik/t) (for now)  
* [toot](https://github.com/ihabunek/toot) (for now)  

## Dependencies for compiling  
* gtk3  
* cairo  
* libnotify  
* pango  
* gdk-pixbuf2  
* atk  
* openssl  

### Ubuntu dependencies  
* libgtk-3-dev  
* libcairo2-dev  
* libnotify-dev  
* libpango1.0-dev  
* libgdk-pixbuf2.0-dev  
* libatk1.0-dev  
* libssl1.0-dev  
* libssl-dev  

### Fedora dependencies  
* gtk3-devel  
* cairo-devel  
* libnotify-devel  
* pango-devel  
* gdk-pixbuf2-devel  
* atk-devel  
* openssl-devel  

## Note: Ubuntu binary may be older  
#### Ubuntu binary tested on Ubuntu 17.04  
#### Regular binary tested on and works on literally anything besides Ubuntu  

## Features  
* Uploads to Twitter and Mastodon  
* Allows taking screenshots and saving them to files  
* Notifications via libnotify  
* GUI works with X11 and Wayland  
* Screenshotting works with X11 and Gnome/Plasma/Sway for Wayland  
* Saves screenshots to a folder in your Pictures directory  

## If the compiled binary doesn't work, then you must manually compile  

### Dependency installation on Fedora  
`dnf install gtk3-devel cairo-devel libnotify-devel pango-devel gdk-pixbuf2-devel atk-devel openssl-devel`  

### Dependency installation on Ubuntu  
`apt install libgtk-3-dev libcairo2-dev libnotify-dev libpango1.0-dev libgdk-pixbuf2.0-dev libatk1.0-dev libssl1.0-dev libssl-dev`  

### Compiling (via Github)  
1. `git clone https://github.com/thebitstick/ShareXin/`  
2. `cargo install`  
3. Login to Twitter and/or Mastodon using `t` and/or `toot`  
4. Explore `--help`  

### Compiling (via Crates.io)  
1. `cargo install sharexin`  
2. Login to Twitter and/or Mastodon using `t` and/or `toot`  
3. Explore `--help`  

## `--help`  

```rust

sharexin 0.4.3 2017-08-03

Usage: sharexin <options> [destination] <image options> [FILE]

Options:
  -h, --help	Display this help message and exit
  -V, --version	Print version info and exit
  -U, --upgrade	Check for new updates

Image Options:
  area		Grab an area of the screen instead of the entire screen
  window	Grab the current window instead of the entire screen
  full		Gran the entire screen
  open		Use a file

Destinations:
  toot		Upload to Mastodon (uses "toot")
  tweet		Upload to Twitter (uses "t")
  imgur		Upload to Imgur
  file		Only save file

Examples:
  sharexin toot
  sharexin tweet full
  sharexin file window
  sharexin toot area
  sharexin imgur open [FILE]

```  

## Language support  

#### English  
#### Français by [@Eleoryth](https://twitter.com/Eleoryth)  
#### Español  
#### Esperanta  
#### 简体中文  
#### 繁體中文  
#### 日本語  
#### 한국어  
#### Deutsche by [@qwertxzy](https://twitter.com/qwertxzy)  

## Known issues  
#### If `sharexin`, `toot`, or `t` aren't found  
Check your $PATH. Your terminal may be able to launch it, but your WM/DE may not.  
If you have a WM like i3, you can add it to your PATH in your `.xprofile`.  

#### If `t` takes forever to send a tweet  
Remember that it's only a Ruby app...  

#### If Rust crashes with `Only Gnome/Plasma/Sway desktops supported for Wayland.`  
Only Gnome, KDE, and Sway desktops are supported at this time. Screenshotting on Wayland is hell, but it makes it "secure" for the current year.  

#### If Rust crashes with `Unable to figure out session type. Check XDG variable.`  
Check your `$XDG_SESSION_TYPE` variable. If it's not x11 or wayland, it crashes as a failsafe.  

#### If Rust crashes with `XDG not found.`  
XDG variables were not found. Check `$DESKTOP_SESSION` and `$XDG_SESSION_TYPE`.  

## Changelog  
#### [0.4.3] - 2017-08-03  
- Bug fixes  
- Cleaner code  

#### [0.4.1] - 2017-08-02  
- Partial Wayland support for Gnome, Plasma, and Sway  
- Gnome-Screenshot used on Gnome  
- Spectacle used on Plasma/KDE  
- Swaygrab used on Sway (i3-clone)  

#### [0.4.0] - 2017-08-01  
- Cursor hidden in all screenshots  
- Removed double shadow  
- Bug fixes  

#### [0.3.9] - 2017-07-31  
- Bug fixes  

#### [0.3.8] - 2017-07-30  
- Better struct management  
- Imgur support! Opens image in browser  
- Changed date variable (had to change it manually everytime  

#### [0.3.7] - 2017-07-29  
- Custom error handles  
- Added experimental `open` Image Option, lets you select an image (animated or not) or possibly a video and send to a destination  
- Update checker implemented properly  
- Fixed service variables, now using structs  
- Readying code for more destinations  
- `auth` option not working at this current moment in time due to immature twitter apis available for Rust  
- `imgur` option hints at what will be the next Destination  

#### [0.3.6] - 2017-07-28  
- Character count now turns red when over the limit  
- You can't send a toot or tweet if it's over the limit  
- Double the ImageMagick shadow  
- No longer using Pipers crate  
- Changed those weird "mort" and "morti" variables  
- Commented most of the code, cleaned up  

#### [0.3.5] - 2017-07-27  
- Added button to check your image  
- Added upgrade checker  
- Added character count  
- Beautified code  
- New command line args  

#### [0.3.4] - 2017-07-26  
- Window screenshot adds shadow  
- Separated main.rs function gui()  

#### [0.3.3] - 2017-07-26  
- New UI thanks to Glade  
- Notifications now have language support  
- Pressing Control+Return sends your message  
- Native Buttons, language changes!  

#### [0.3.2] - 2017-07-25  
- Cleaned help messages, used GNU coreutil messages for some items  

#### [0.3.1] - 2017-07-25  
- Separated main.rs into {main, help, file, send}.rs, less scrolling  

#### [0.3.0] - 2017-07-25  
- Die Deutsche Sprache!  
- Falsche Kompilierungsdatum  

#### [0.2.9] - 2017-07-25  
- Multaj Lingovj! Added French, Spanish, Esperanto, and Japanese translations for --help!  

#### [0.2.8] - 2017-07-25  
- Maim replacing Gnome-screenshot  
- Shadow added to area screenshots using ImageMagick  
- Notification adds back tweet text  

#### [0.2.7] - 2017-07-24  
- Notifications via libnotify (bye bye dbus)  
- Username gotten by $USER var, rather than an entire library (thanks std!)  

#### [0.2.6] - 2017-07-23  
- Forgot to update the version # for 0.2.5 from 0.2.4 and Crates wouldn't allow a reupload so....  

#### [0.2.5] - 2017-07-23  
- Better word wrap  
- Better temp dir  
- Notification actually shows image now  

#### [0.2.4] - 2017-07-21  
- Added version info  
- Made --help prettier  

#### [0.2.3] - 2017-07-21  
- Send button now says Toot or Tweet depending on where you're going  
- TextView no longer accepts tabs  

#### [0.2.2] - 2017-07-21  
- TextView now word wraps  
- Ability to simply just tweet without an image  
- Mort  

#### [0.2.1] - 2017-07-20  
- Centered window (why isn't .set_position() IN THE DOCS)  

#### [0.2.0] - 2017-07-20  
- Uh, if you're haven problems with t not loadin', check your $PATH  

#### [0.1.0] - 2017-07-19  
- Bug fixes and improvements  

#### [0.0.0] - 2017-07-19  
##### Added  
- First commit  
